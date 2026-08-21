use std::time::Duration;

use bytes::{Bytes, BytesMut};
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};
use prost::Message as _;
use tokio::time::sleep;

use crate::{Error, HyperClient, Result, proto};

const URI: &str = "https://android.clients.google.com/checkin";
const CHROME_VERSION: &str = "149.0.7827.200";
const SLEEP_DURATION: Duration = Duration::from_secs(3);

pub(crate) struct CheckInResponse {
    pub android_id: Option<u64>,
    pub security_token: Option<u64>,
}

pub(crate) struct CheckInRequest {}
impl CheckInRequest {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<CheckInResponse> {
        let request = proto::AndroidCheckinRequest {
            checkin: proto::AndroidCheckinProto {
                r#type: Some(3),
                chrome_build: Some(proto::ChromeBuildProto {
                    platform: Some(3),
                    chrome_version: Some(CHROME_VERSION.to_owned()),
                    channel: Some(1),
                }),
                ..Default::default()
            },
            version: Some(3),
            user_serial_number: Some(0),
            ..Default::default()
        };

        let mut payload = BytesMut::with_capacity(request.encoded_len());
        request.encode(&mut payload)?;

        let bytes = self.send_raw(client, payload.freeze()).await?;
        let response = proto::AndroidCheckinResponse::decode(bytes)?;

        // Google servers are slow as fuck, without waiting the next registration request will fail
        // 99% of time returning PHONE_REGISTRATION_ERROR
        sleep(SLEEP_DURATION).await;

        Ok(CheckInResponse {
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
