use kafka_protocol_serde::kafka_serialize::KafkaSerialize;
use kafka_protocol_serde::types::CompactArray;
use kafka_protocol_serde::{kafka_deserialize::KafkaDeserialize, types::TagBuffer};
use kafka_protocol_serde_macros::{KafkaDeserialize, KafkaSerialize};

use crate::request::{Request, RequestHeader};

#[derive(KafkaDeserialize)]
pub struct ApiVersionsRequestBody;

#[derive(KafkaSerialize, Clone)]
struct ApiKey {
    api_key: i16,
    min_version: i16,
    max_version: i16,
    tag_buffer: TagBuffer,
}

static SUPPORTED_API_KEYS: [ApiKey; 2] = [
    ApiKey {
        api_key: 18,
        min_version: 4,
        max_version: 4,
        tag_buffer: TagBuffer,
    },
    ApiKey {
        api_key: 1,
        min_version: 16,
        max_version: 16,
        tag_buffer: TagBuffer,
    },
];

#[derive(KafkaSerialize)]
pub struct ApiVersionsResponseBody {
    error_code: i16,
    api_keys: CompactArray<ApiKey>,
    throttle_time_ms: i32,
    tag_buffer: TagBuffer,
}

pub fn handle_api_versions(header: RequestHeader) -> ApiVersionsResponseBody {
    if header.request_api_version != 4 {
        return ApiVersionsResponseBody {
            error_code: 35,
            api_keys: CompactArray(vec![]),
            throttle_time_ms: 0,
            tag_buffer: TagBuffer,
        };
    }

    ApiVersionsResponseBody {
        error_code: 0,
        api_keys: CompactArray(SUPPORTED_API_KEYS.to_vec()),
        throttle_time_ms: 0,
        tag_buffer: TagBuffer,
    }
}
