pub mod kafka_deserialize;
pub mod kafka_serialize;
pub mod types;

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use std::io;

    use crate::kafka_serialize::KafkaSerialize;
    use crate::types::{CompactArray, TagBuffer};
    use crate::{kafka_deserialize::KafkaDeserialize, kafka_serialize::serialize_message};
    use kafka_protocol_serde_macros::{KafkaDeserialize, KafkaSerialize};

    #[derive(KafkaDeserialize, KafkaSerialize, Debug)]
    struct Message {
        field1: u32,
        field2: u32,
        field3: CompactArray<u32>,
    }

    #[test]
    fn test_simple_deserialize() {
        let mut bin = 54usize.to_be_bytes().to_vec();
        bin.push(3u8);
        bin.append(&mut [1u32, 10u32].iter().flat_map(|x| x.to_be_bytes()).collect());

        let mut reader = io::BufReader::new(&bin[..]);

        let message = Message::kafka_deserialize(&mut reader);
        assert!(message.is_ok());

        let message = message.unwrap();
        assert_eq!(message.field1, 0);
        assert_eq!(message.field2, 54);
        assert_eq!(message.field3.0.len(), 2);
        assert_eq!(message.field3.0[0], 1);
        assert_eq!(message.field3.0[1], 10);
    }

    #[test]
    fn test_simple_serialize() {
        let message = Message {
            field1: 31,
            field2: 40,
            field3: CompactArray(vec![1, 4, 20, 34]),
        };

        let serialized = serialize_message::<Vec<u8>>(&message);
        assert!(serialized.is_ok());

        let serialized = serialized.unwrap();
        assert_eq!(serialized, {
            let mut expected: Vec<u8> = vec![31u32, 40u32]
                .into_iter()
                .flat_map(|x| x.to_be_bytes())
                .collect();
            expected.push(5u8);
            let field3 = vec![1u32, 4u32, 20u32, 34u32];
            expected.append(
                &mut field3
                    .into_iter()
                    .flat_map(|x| x.to_be_bytes())
                    .collect::<Vec<u8>>(),
            );

            expected
        });

        assert_eq!(message.kafka_byte_len(), serialized.len());
    }

    #[derive(KafkaDeserialize, Debug)]
    pub struct RequestHeader {
        pub request_api_key: i16,
        pub request_api_version: i16,
        pub correlation_id: i32,
        pub client_id: Option<String>,
        pub tagged_fields: TagBuffer,
    }

    #[test]
    fn test_deserialize_header() {
        let bin = [
            0, 18, 0, 4, 92, 127, 67, 142, 0, 9, 107, 97, 102, 107, 97, 45, 99, 108, 105, 0, 10,
            107, 97, 102, 107, 97, 45, 99, 108, 105, 4, 48, 46, 49, 0,
        ];

        let mut reader = io::BufReader::new(&bin[..]);

        let message = RequestHeader::kafka_deserialize(&mut reader);
        assert!(message.is_ok());
    }
}
