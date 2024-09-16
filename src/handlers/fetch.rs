#![allow(dead_code)]

use kafka_protocol_serde::kafka_deserialize::KafkaDeserialize;
use kafka_protocol_serde::kafka_serialize::KafkaSerialize;
use kafka_protocol_serde::types::{CompactArray, CompactRecords, CompactString, TagBuffer, UUIDv4};
use kafka_protocol_serde_macros::{KafkaDeserialize, KafkaSerialize};

use crate::request::RequestHeader;

#[derive(KafkaDeserialize)]
struct Topic {
    topic_id: UUIDv4,
    partitions: CompactArray<ReqTopicPartition>,
    tag_buffer: TagBuffer,
}

#[derive(KafkaDeserialize)]
struct ReqTopicPartition {
    partition: i32,
    current_leader_epoch: i32,
    fetch_offset: i64,
    last_fetched_epoch: i32,
    log_start_offset: i64,
    partition_max_bytes: i32,
    tag_buffer: TagBuffer,
}

#[derive(KafkaDeserialize)]
struct ForgottenTopicsData {
    topic_id: UUIDv4,
    partitions: CompactArray<i32>,
    tag_buffer: TagBuffer,
}

#[derive(KafkaDeserialize)]
pub struct FetchRequestBody {
    max_wait_ms: i32,
    min_bytes: i32,
    max_bytes: i32,
    isolation_level: i8,
    session_id: i32,
    session_epoch: i32,
    topics: CompactArray<Topic>,
    forgotten_topics_data: CompactArray<ForgottenTopicsData>,
    rack_id: CompactString,
    tag_buffer: TagBuffer,
}

#[derive(KafkaSerialize)]
struct AbortedTransaction {
    producer_id: i64,
    first_offset: i64,
    tag_buffer: TagBuffer,
}

#[derive(KafkaSerialize)]
struct ResTopicPartition {
    partition_index: i32,
    error_code: i16,
    high_watermark: i64,
    last_stable_offset: i64,
    log_start_offset: i64,
    aborted_transactions: CompactArray<AbortedTransaction>,
    preferred_read_replica: i32,
    records: CompactRecords,
    tag_buffer: TagBuffer,
}

#[derive(KafkaSerialize)]
struct Response {
    topic_id: UUIDv4,
    partitions: CompactArray<ResTopicPartition>,
    tag_buffer: TagBuffer,
}

#[derive(KafkaSerialize)]
pub struct FetchResponseBody {
    throttle_time_ms: i32,
    error_code: i16,
    session_id: i32,
    responses: CompactArray<Response>,
    tag_buffer: TagBuffer,
}

pub fn handle_fetch(_header: RequestHeader, body: FetchRequestBody) -> FetchResponseBody {
    let responses = body
        .topics
        .0
        .into_iter()
        .map(fetch_topic)
        .collect::<Vec<_>>();

    FetchResponseBody {
        throttle_time_ms: 0,
        error_code: 0,
        session_id: 0,
        responses: CompactArray(responses),
        tag_buffer: TagBuffer,
    }
}

fn fetch_topic(topic: Topic) -> Response {
    Response {
        tag_buffer: TagBuffer,
        topic_id: topic.topic_id,
        partitions: CompactArray(vec![ResTopicPartition {
            error_code: 100,
            partition_index: 0,
            high_watermark: 0,
            last_stable_offset: 0,
            log_start_offset: 0,
            aborted_transactions: CompactArray(vec![]),
            preferred_read_replica: 0,
            records: CompactRecords,
            tag_buffer: TagBuffer,
        }]),
    }
}
