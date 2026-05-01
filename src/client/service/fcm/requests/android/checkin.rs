#[expect(warnings)]
mod checkin_proto {
    include!(concat!(env!("OUT_DIR"), "/checkin_proto.rs"));
}

use bytes::{Bytes, BytesMut};
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};
use prost::Message as _;

use crate::client::{FcmError, HyperClient};
use crate::{Error, Result};

const URI: &str = "https://android.clients.google.com/checkin";

pub(crate) struct Response {
    pub android_id: Option<u64>,
    pub security_token: Option<u64>,
}

impl TryFrom<Response> for (u64, u64) {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        let android_id = value
            .android_id
            .ok_or(Error::ConversionError("android_id is missing".to_owned()))?;
        let security_token = value.security_token.ok_or(Error::ConversionError(
            "security_token is missing".to_owned(),
        ))?;

        Ok((android_id, security_token))
    }
}

pub(crate) struct CheckinRequest {}
impl CheckinRequest {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<Response> {
        let request = checkin_proto::AndroidCheckinRequest {
            checkin: checkin_proto::AndroidCheckinProto {
                r#type: Some(3),
                chrome_build: Some(checkin_proto::ChromeBuildProto {
                    platform: Some(2),
                    chrome_version: Some("63.0.3234.0".to_owned()),
                    channel: Some(1),
                }),
                ..Default::default()
            },
            version: Some(3),
            user_serial_number: Some(0),
            ..Default::default()
        };

        let mut payload = BytesMut::with_capacity(request.encoded_len());
        request.encode(&mut payload).map_err(FcmError::from)?;

        let bytes = self.send_raw(client, payload.freeze()).await?;
        let response = checkin_proto::AndroidCheckinResponse::decode(bytes).map_err(|error| {
            tracing::error!(?error, "failed to parse Android checkin response");
            FcmError::from(error)
        })?;

        Ok(Response {
            android_id: response.android_id,
            security_token: response.security_token,
        })
    }

    async fn send_raw(&self, client: &HyperClient, body: Bytes) -> Result<Bytes> {
        let request = http::Request::builder()
            .uri(URI)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/x-protobuf")
            .body(Either::Left(Full::new(body)))?;

        let response = client.request(request).await?;

        let status = response.status();
        if status.is_success() {
            Ok(response.collect().await?.to_bytes())
        } else {
            Err(Error::UnsuccessfulResponse(status))
        }
    }
}
