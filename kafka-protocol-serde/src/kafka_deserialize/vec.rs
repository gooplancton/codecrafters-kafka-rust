use crate::types::CompactArray;

use super::{utils::read_varint, KafkaDeserialize};

impl<T> KafkaDeserialize for CompactArray<T>
where
    T: KafkaDeserialize,
{
    fn kafka_deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let len = read_varint(reader)? - 1;

        let mut inner: Vec<T> = vec![];
        inner.reserve_exact(len as usize);

        for _ in 0..len {
            let item = T::kafka_deserialize(reader)?;
            inner.push(item);
        }

        Ok(CompactArray(inner))
    }
}
