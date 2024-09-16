pub fn read_varint<R: std::io::Read>(reader: &mut R) -> std::io::Result<u64> {
    let mut res: u64 = 0;
    let mut buf = [0u8; 1];
    reader.read_exact(&mut buf)?;

    res += ((buf[0] << 1) >> 1) as u64;

    let mut offset = 1;
    while buf[0] >> 7 == 1 {
        reader.read_exact(&mut buf)?;
        let chunk = ((buf[0] << 1) >> 1) as usize;
        let chunk: u64 = (chunk << (offset * 7)) as u64;

        res += chunk;
        offset += 1;
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use std::io::BufReader;

    use super::read_varint;

    #[test]
    fn test_read_single_byte() {
        let bin = [0b0000_0001];
        let mut reader = BufReader::new(bin.as_slice());
        let num = read_varint(&mut reader);

        assert!(num.is_ok());
        assert_eq!(num.unwrap(), 1)
    }

    #[test]
    fn test_read_multiple_bytes() {
        let bin = [0b1001_0110, 0b0000_0001];
        let mut reader = BufReader::new(bin.as_slice());
        let num = read_varint(&mut reader);

        assert!(num.is_ok());
        assert_eq!(num.unwrap(), 150)
    }
}
