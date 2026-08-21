use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use bytes::Bytes;
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};
use serde_json::json;

use super::URI;
use crate::requests::Token;
use crate::{Error, HyperClient, Result};

const ENDPOINT: &str = "https://fcm.googleapis.com/fcm";

pub(crate) struct RegistrationRequest<'a> {
    project_id: &'a str,
    api_key: &'a str,
    vapid_key: &'a str,
    installation_auth_token: &'a str,
    gcm_token: &'a str,
    public_key: &'a [u8],
    auth_secret: &'a [u8],
}
impl<'a> RegistrationRequest<'a> {
    pub(crate) fn new(
        project_id: &'a str,
        api_key: &'a str,
        vapid_key: &'a str,
        installation_auth_token: &'a str,
        gcm_token: &'a str,
        public_key: &'a [u8],
        auth_secret: &'a [u8],
    ) -> Self {
        Self {
            project_id,
            api_key,
            vapid_key,
            installation_auth_token,
            gcm_token,
            public_key,
            auth_secret,
        }
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<Token> {
        let endpoint = format!("{ENDPOINT}/send/{}", self.gcm_token);
        let public_key = URL_SAFE_NO_PAD.encode(self.public_key);
        let auth_secret = URL_SAFE_NO_PAD.encode(self.auth_secret);

        let request = json!({
            "web": {
                "endpoint": endpoint,
                "auth": auth_secret,
                "p256dh": public_key,
                "applicationPubKey": self.vapid_key,
            }
        });

        let payload = serde_json::to_vec(&request)?;
        let bytes = self.send_raw(client, Bytes::from(payload)).await?;

        let response = serde_json::from_slice(&bytes)?;
        Ok(response)
    }

    async fn send_raw(&self, client: &HyperClient, body: Bytes) -> Result<Bytes> {
        let uri = format!("{URI}/projects/{}/registrations", self.project_id);

        let request = http::Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .header("x-goog-api-key", self.api_key)
            .header(
                "x-goog-firebase-installations-auth",
                self.installation_auth_token,
            )
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
