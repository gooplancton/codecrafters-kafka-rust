use api_versions::handle_api_versions;
use fetch::handle_fetch;
use kafka_protocol_serde::{kafka_serialize::{serialize_message, KafkaSerialize}, types::TagBuffer};

use crate::{
    request::Request,
    response::{Response, ResponseHeaderV0, ResponseHeaderV1},
};

pub mod api_versions;
pub mod fetch;

pub fn handle_request(req: Request) -> anyhow::Result<Response> {
    match req {
        Request::ApiVersionsRequest { header, body: _ } => {
            let res_header = ResponseHeaderV0 {
                correlation_id: header.correlation_id,
            };

            let res_body = handle_api_versions(header);

            Ok(Response::ApiVersionsResponse {
                header: res_header,
                body: res_body,
            })
        }
        Request::FetchRequest { header, body } => {
            let res_header = ResponseHeaderV1 {
                correlation_id: header.correlation_id,
                tag_buffer: TagBuffer,
            };

            let res_body = handle_fetch(header, body);

            Ok(Response::FetchResponse {
                header: res_header,
                body: res_body,
            })
        }
    }
}
