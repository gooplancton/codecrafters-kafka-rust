use std::io::{BufWriter, Write};

use super::KafkaSerialize;

macro_rules! impl_numerics {
    ( $(($num_ty:ty, $byte_size:expr) );* ) => {
        $(impl KafkaSerialize for $num_ty {
            fn kafka_serialize<W: std::io::Write>(&self, writer: &mut BufWriter<W>) -> std::io::Result<()> {
                writer.write_all(&self.to_be_bytes()[..])?;

                Ok(())
            }

            fn kafka_byte_len(&self) -> usize {
                $byte_size
            }
        })*
    };
}

impl_numerics!((u128, 16); (u64, 8); (i64, 8); (u32, 4); (i32, 4); (u16, 2); (i16, 2); (u8, 1); (i8, 1));

