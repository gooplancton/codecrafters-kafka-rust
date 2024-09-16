use std::io::{BufWriter, Write};

pub fn write_varint<W: std::io::Write>(writer: &mut BufWriter<W>, num: u64) -> std::io::Result<()> {
    let num_bytes = num.to_le();
    let mut offset = 1;
    let mut chunk: u8 = (num_bytes >> (7 * (offset - 1)) << (64 - 7 * offset) >> (64 - 7 * offset))
        .try_into()
        .unwrap();

    let mut next_chunk: u8 = (num_bytes >> (7 * (offset)) << (64 - 7 * (offset + 1))
        >> (64 - 7 * (offset + 1)))
        .try_into()
        .unwrap();

    if next_chunk == 0 {
        writer.write_all(&[chunk])?;
        return writer.flush();
    }

    loop {
        writer.write_all(&[(1 << 7) as u8 + chunk])?;

        offset += 1;

        chunk = next_chunk;
        next_chunk = (num_bytes >> (7 * (offset)) << (64 - 7 * (offset + 1))
            >> (64 - 7 * (offset + 1)))
            .try_into()
            .unwrap();

        if next_chunk == 0 {
            writer.write_all(&[chunk])?;
            return writer.flush();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, BufWriter};

    use super::write_varint;
    use crate::kafka_deserialize::utils::read_varint;

    #[test]
    fn test_write_varint() {
        let mut buf: Vec<u8> = vec![];
        let mut writer = BufWriter::new(&mut buf);

        let res = write_varint(&mut writer, 150u64);
        assert!(res.is_ok());

        let inner = writer.into_inner().unwrap();
        let mut reader = BufReader::new(inner.as_slice());

        let num = read_varint(&mut reader);
        assert!(num.is_ok());

        let num = num.unwrap();

        assert_eq!(num, 150u64);
    }
}
