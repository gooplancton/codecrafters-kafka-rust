pub mod numeric;
pub mod vec;
pub mod misc;
pub mod string;
pub mod utils;

use std::io::Read;

pub trait KafkaDeserialize: Sized {
    fn kafka_deserialize<R: Read>(reader: &mut R) -> std::io::Result<Self>;
}

