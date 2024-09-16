use crate::types::CompactString;

use super::{utils::read_varint, KafkaDeserialize};

impl KafkaDeserialize for Option<String> {
    fn kafka_deserialize<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut len_buf = [0u8; 2];
        reader.read_exact(&mut len_buf)?;
        let len = i16::from_be_bytes(len_buf);
        if len == 0 {
            return Ok(None);
        }

        let mut chars = vec![0u8; len as usize];
        reader.read_exact(&mut chars)?;

        let res = String::from_utf8_lossy(&chars[..]);
        Ok(Some(res.to_string()))
    }
}

impl KafkaDeserialize for CompactString {
    fn kafka_deserialize<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len = read_varint(reader)? - 1;

        let mut chars = vec![0u8; len as usize];
        reader.read_exact(&mut chars)?;

        let res = String::from_utf8_lossy(&chars[..]);
        Ok(CompactString(res.to_string()))
    }
}

