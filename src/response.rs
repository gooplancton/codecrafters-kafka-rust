use std::{
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
};

use kafka_protocol_serde::{kafka_serialize::KafkaSerialize, types::TagBuffer};
use kafka_protocol_serde_macros::KafkaSerialize;

use crate::handlers::{api_versions::ApiVersionsResponseBody, fetch::FetchResponseBody};

#[derive(KafkaSerialize)]
pub struct ResponseHeaderV0 {
    pub correlation_id: i32,
}
#[derive(KafkaSerialize)]
pub struct ResponseHeaderV1 {
    pub correlation_id: i32,
    pub tag_buffer: TagBuffer,
}

#[derive(KafkaSerialize)]
pub enum Response {
    ApiVersionsResponse {
        header: ResponseHeaderV0,
        body: ApiVersionsResponseBody,
    },
    FetchResponse {
        header: ResponseHeaderV1,
        body: FetchResponseBody
    }
}

pub trait ResponseWriter {
    fn write_response(&mut self, res: Response) -> std::io::Result<()>;
}

impl ResponseWriter for BufWriter<TcpStream> {
    fn write_response(&mut self, res: Response) -> std::io::Result<()> {
        let res_byte_len = res.kafka_byte_len() as u32;
        self.write_all(&res_byte_len.to_be_bytes()[..])?;

        res.kafka_serialize(self)
    }
}
