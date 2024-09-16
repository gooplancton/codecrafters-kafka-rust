use crate::types::{CompactRecords, TagBuffer};

use super::KafkaDeserialize;

impl KafkaDeserialize for TagBuffer {
    fn kafka_deserialize<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
        // NOTE: not implemented in this codecrafters challenge
        let mut buf = [0u8];
        reader.read_exact(&mut buf[..])?;

        Ok(TagBuffer)
    }
}

impl KafkaDeserialize for CompactRecords {
    fn kafka_deserialize<R: std::io::prelude::Read>(reader: &mut R) -> std::io::Result<Self> {
        // NOTE: not implemented in this codecrafters challenge
        let mut buf = (-1i32).to_be_bytes();
        reader.read_exact(&mut buf[..])?;

        Ok(CompactRecords)
    }
}
