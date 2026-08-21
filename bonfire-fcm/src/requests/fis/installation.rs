use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use bytes::Bytes;
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};
use rand::random;
use serde::Deserialize;
use serde_json::json;

use super::{HEARTBEATS, URI};
use crate::requests::Token;
use crate::{Error, HyperClient, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InstallationResponse {
    #[serde(rename = "fid")]
    pub id: String,
    pub auth_token: Token,
    pub refresh_token: String,
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

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<InstallationResponse> {
        let request = json!({
            "fid": Self::generate_id(),
            "authVersion": "FIS_v2",
            "appId": self.app_id,
            "sdkVersion": "w:0.4.8",
        });

        let payload = serde_json::to_vec(&request)?;
        let bytes = self.send_raw(client, Bytes::from(payload)).await?;

        let response = serde_json::from_slice(&bytes)?;
        Ok(response)
    }

    fn generate_id() -> String {
        let mut bytes: [u8; 17] = random();
        bytes[0] = 0b0111_0000 + (bytes[0] % 0b0001_0000);
        URL_SAFE_NO_PAD.encode(bytes)
    }

    async fn send_raw(&self, client: &HyperClient, body: Bytes) -> Result<Bytes> {
        let uri = format!("{URI}/projects/{}/installations", self.project_id);

        let request = http::Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .header("x-firebase-client", &**HEARTBEATS)
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
