use super::KafkaDeserialize;

macro_rules! impl_numerics {
    ( $(($num_ty:ty, $byte_size:expr) );* ) => {
        $(impl KafkaDeserialize for $num_ty {
            fn kafka_deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
                let mut buf = [0u8; $byte_size];
                reader.read_exact(&mut buf)?;
                let res = <$num_ty>::from_be_bytes(buf);

                Ok(res)
            }
        })*
    };
}

impl_numerics!((u128, 16); (u64, 8); (i64, 8); (u32, 4); (i32, 4); (u16, 2); (i16, 2); (u8, 1); (i8, 1));

