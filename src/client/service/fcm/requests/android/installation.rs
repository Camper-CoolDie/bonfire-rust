use base64::Engine as _;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use bytes::Bytes;
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};
use rand::random;
use serde::Deserialize;
use serde_json::json;

use crate::client::HyperClient;
use crate::{Error, Result};

const URI: &str = "https://firebaseinstallations.googleapis.com/v1";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    auth_token: InstallationAuthToken,
}

#[derive(Deserialize)]
struct InstallationAuthToken {
    #[serde(rename = "token")]
    value: String,
}

pub(crate) struct InstallationRequest<'a> {
    project_id: &'a str,
    app_id: &'a str,
    api_key: &'a str,
}
impl<'a> InstallationRequest<'a> {
    pub(crate) fn new(project_id: &'a str, app_id: &'a str, api_key: &'a str) -> Self {
        Self {
            project_id,
            app_id,
            api_key,
        }
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<String> {
        let request = json!({
            "app_id": self.app_id,
            "auth_version": "FIS_v2",
            "fid": Self::generate_installation_id(),
            "sdk_version": "w:0.6.4",
        });

        let payload = serde_json::to_vec(&request)?;
        let bytes = self.send_raw(client, Bytes::from(payload)).await?;

        let response = serde_json::from_slice::<Response>(&bytes).inspect_err(|error| {
            tracing::error!(?error, "failed to parse installation response");
        })?;
        Ok(response.auth_token.value)
    }

    fn generate_installation_id() -> String {
        let mut bytes: [u8; 17] = random();
        bytes[0] = 0b0111_0000 + (bytes[0] % 0b0001_0000);
        BASE64_URL_SAFE_NO_PAD.encode(bytes)
    }

    async fn send_raw(&self, client: &HyperClient, body: Bytes) -> Result<Bytes> {
        let uri = format!("{URI}/projects/{}/installations", self.project_id);

        let heartbeats = json!({"heartbeats": [], "version": 2});
        let encoded_heartbeats = BASE64_URL_SAFE_NO_PAD.encode(serde_json::to_vec(&heartbeats)?);

        let request = http::Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .header("x-firebase-client", encoded_heartbeats)
            .header("x-goog-api-key", self.api_key)
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
