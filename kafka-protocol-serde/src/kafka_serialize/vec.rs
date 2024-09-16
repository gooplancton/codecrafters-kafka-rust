use std::io::{BufWriter, Write};

use super::utils::write_varint;
use crate::types::CompactArray;

use super::KafkaSerialize;

impl<T> KafkaSerialize for CompactArray<T>
where
    T: KafkaSerialize,
{
    fn kafka_serialize<W: Write>(&self, writer: &mut BufWriter<W>) -> std::io::Result<()> {
        write_varint(writer, (self.0.len() + 1) as u64)?;

        for item in self.0.iter() {
            item.kafka_serialize(writer)?;
        }

        Ok(())
    }

    fn kafka_byte_len(&self) -> usize {
        1 + if self.0.is_empty() {
            0
        } else {
            self.0[0].kafka_byte_len() * self.0.len()
        }
    }
}
