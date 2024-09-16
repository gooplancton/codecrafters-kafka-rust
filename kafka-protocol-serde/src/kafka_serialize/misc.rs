use std::io::Write;

use crate::types::{CompactRecords, TagBuffer};

use super::KafkaSerialize;

impl KafkaSerialize for TagBuffer {
    fn kafka_byte_len(&self) -> usize {
        1
    }

    fn kafka_serialize<W: std::io::prelude::Write>(
        &self,
        writer: &mut std::io::BufWriter<W>,
    ) -> std::io::Result<()> {
        // NOTE: not implemented in this codecrafters challenge
        writer.write_all([0u8].as_slice())
    }
}

impl KafkaSerialize for CompactRecords {
    fn kafka_byte_len(&self) -> usize {
        1
    }

    fn kafka_serialize<W: std::io::prelude::Write>(
        &self,
        writer: &mut std::io::BufWriter<W>,
    ) -> std::io::Result<()> {
        // NOTE: not implemented in this codecrafters challenge
        writer.write_all([0u8].as_slice())
    }
}
