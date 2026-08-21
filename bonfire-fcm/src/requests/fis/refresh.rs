use bytes::Bytes;
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};
use serde_json::json;

use super::{HEARTBEATS, URI};
use crate::requests::Token;
use crate::{Error, HyperClient, Result};

pub(crate) struct RefreshRequest<'a> {
    project_id: &'a str,
    app_id: &'a str,
    api_key: &'a str,
    id: &'a str,
    refresh_token: &'a str,
}
impl<'a> RefreshRequest<'a> {
    pub(crate) fn new(
        project_id: &'a str,
        app_id: &'a str,
        api_key: &'a str,
        id: &'a str,
        refresh_token: &'a str,
    ) -> Self {
        Self {
            project_id,
            app_id,
            api_key,
            id,
            refresh_token,
        }
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<Token> {
        let request = json!({
            "installation": {
                "appId": self.app_id,
                "sdkVersion": "w:0.4.8",
            },
        });

        let payload = serde_json::to_vec(&request)?;
        let bytes = self.send_raw(client, Bytes::from(payload)).await?;

        let response = serde_json::from_slice(&bytes)?;
        Ok(response)
    }

    async fn send_raw(&self, client: &HyperClient, body: Bytes) -> Result<Bytes> {
        let uri = format!(
            "{URI}/projects/{}/installations/{}/authTokens:generate",
            self.project_id, self.id
        );
        let auth = format!("FIS_v2 {}", self.refresh_token);

        let request = http::Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::AUTHORIZATION, auth)
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
