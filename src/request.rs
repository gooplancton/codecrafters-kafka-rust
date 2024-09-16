#![allow(dead_code)]

use std::{
    io::{BufReader, Read},
    net::TcpStream,
};

use anyhow::bail;
use kafka_protocol_serde::{kafka_deserialize::KafkaDeserialize, types::TagBuffer};
use kafka_protocol_serde_macros::KafkaDeserialize;

use crate::handlers::{api_versions::ApiVersionsRequestBody, fetch::FetchRequestBody};

#[derive(KafkaDeserialize, Debug)]
pub struct RequestHeader {
    pub request_api_key: i16,
    pub request_api_version: i16,
    pub correlation_id: i32,
    pub client_id: Option<String>,
    pub tagged_fields: TagBuffer,
}

pub enum Request {
    ApiVersionsRequest {
        header: RequestHeader,
        body: ApiVersionsRequestBody,
    },
    FetchRequest {
        header: RequestHeader,
        body: FetchRequestBody,
    },
}

pub trait RequestParser {
    fn parse_request(&mut self) -> anyhow::Result<Request>;
}

impl RequestParser for BufReader<TcpStream> {
    fn parse_request(&mut self) -> anyhow::Result<Request> {
        let mut message_len = [0u8; 4];
        self.read_exact(&mut message_len)?;
        let message_len = u32::from_be_bytes(message_len);

        let mut message = vec![0u8; message_len as usize];
        self.read_exact(&mut message[..])?;

        let mut message_reader = BufReader::new(&message[..]);
        let header = RequestHeader::kafka_deserialize(&mut message_reader)?;

        let api_key = header.request_api_key;
        let req = match api_key {
            1 => Request::FetchRequest {
                header,
                body: FetchRequestBody::kafka_deserialize(&mut message_reader)?,
            },
            18 => Request::ApiVersionsRequest {
                header,
                body: ApiVersionsRequestBody,
            },
            _ => bail!("unrecognized api key: {}", api_key),
        };

        Ok(req)
    }
}
