use base64::Engine;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use bytes::Bytes;
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};
use serde::Deserialize;
use serde_json::json;

use super::REGISTRATION_URI;
use crate::client::{FcmError, HyperClient};
use crate::{Error, Result};

const ENDPOINT_URI: &str = "https://fcm.googleapis.com/fcm";

#[derive(Deserialize)]
pub(crate) struct Response {
    pub token: String,
    #[serde(skip)]
    pub public_key: String,
    #[serde(skip)]
    pub private_key: String,
    #[serde(skip)]
    pub auth_secret: String,
}

pub(crate) struct RegisterRequest<'a> {
    project_id: &'a str,
    api_key: &'a str,
    vapid_key: Option<&'a str>,
    installation_auth_token: &'a str,
    gcm_token: &'a str,
}
impl<'a> RegisterRequest<'a> {
    pub(crate) fn new(
        project_id: &'a str,
        api_key: &'a str,
        vapid_key: Option<&'a str>,
        installation_auth_token: &'a str,
        gcm_token: &'a str,
    ) -> Self {
        Self {
            project_id,
            api_key,
            vapid_key,
            installation_auth_token,
            gcm_token,
        }
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<Response> {
        let endpoint = format!("{ENDPOINT_URI}/send/{}", self.gcm_token);

        let (key_pair, auth_secret) =
            ece::generate_keypair_and_auth_secret().map_err(FcmError::from)?;
        let key_components = key_pair.raw_components().map_err(FcmError::from)?;
        let public_key = BASE64_URL_SAFE_NO_PAD.encode(key_components.public_key());
        let private_key = BASE64_URL_SAFE_NO_PAD.encode(key_components.private_key());
        let auth_secret = BASE64_URL_SAFE_NO_PAD.encode(auth_secret);

        let request = json!({
            "web": {
                "applicationPubKey": self.vapid_key,
                "endpoint": endpoint,
                "auth": auth_secret,
                "p256dh": public_key,
            }
        });

        let payload = serde_json::to_vec(&request)?;
        let bytes = self.send_raw(client, Bytes::from(payload)).await?;

        let mut response = serde_json::from_slice::<Response>(&bytes).inspect_err(|error| {
            tracing::error!(?error, "failed to parse registration response");
        })?;

        response.public_key = public_key;
        response.private_key = private_key;
        response.auth_secret = auth_secret;
        Ok(response)
    }

    async fn send_raw(&self, client: &HyperClient, body: Bytes) -> Result<Bytes> {
        let uri = format!(
            "{REGISTRATION_URI}/projects/{}/registrations",
            self.project_id
        );

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
