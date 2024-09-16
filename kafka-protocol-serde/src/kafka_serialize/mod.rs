use std::io::{self, BufWriter, Write};

pub mod misc;
pub mod numeric;
pub mod utils;
pub mod vec;

pub trait KafkaSerialize: Sized {
    fn kafka_byte_len(&self) -> usize;
    fn kafka_serialize<W: Write>(&self, writer: &mut BufWriter<W>) -> io::Result<()>;
}

pub fn serialize_message<W: Write>(message: &impl KafkaSerialize) -> io::Result<Vec<u8>> {
    let buf = vec![];
    let mut writer = BufWriter::new(buf);
    message.kafka_serialize(&mut writer)?;

    Ok(writer.into_inner()?)
}
