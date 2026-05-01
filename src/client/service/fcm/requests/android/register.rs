use std::collections::HashMap;

use bytes::Bytes;
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};

use crate::client::{FcmError, HyperClient};
use crate::{Error, Result};

const SERVER_KEY: &str =
    "BDOU99-h67HcA6JeFXHbSNMu7e2yNNu3RzoMj8TM4W88jITfq7ZmPvIM1Iv-4_l2LxQcYwhqby2xGpWwzjfAnG4";

const URI: &str = "https://android.clients.google.com/c2dm/register3";

pub(crate) struct RegisterRequest<'a> {
    app_id: &'a str,
    android_id: u64,
    security_token: u64,
}
impl<'a> RegisterRequest<'a> {
    pub(crate) fn new(app_id: &'a str, android_id: u64, security_token: u64) -> Self {
        Self {
            app_id,
            android_id,
            security_token,
        }
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<String> {
        let android_id = self.android_id.to_string();

        let mut params = HashMap::with_capacity(4);
        params.insert("app", "org.chromium.linux");
        params.insert("X-subtype", self.app_id);
        params.insert("device", &android_id);
        params.insert("sender", SERVER_KEY);
        let payload = serde_urlencoded::to_string(&params).map_err(FcmError::from)?;

        let bytes = self.send_raw(client, Bytes::from(payload)).await?;
        let response =
            serde_urlencoded::from_bytes::<HashMap<&str, String>>(&bytes).map_err(|error| {
                tracing::error!(?error, "failed to parse Android registration response");
                FcmError::from(error)
            })?;

        if response.len() == 1 {
            match response.into_iter().next() {
                Some(("token", token)) => Ok(token),
                Some(("Error", error)) => Err(FcmError::AndroidRegistrationError(error).into()),
                _ => Err(FcmError::InvalidAndroidRegistrationResponse.into()),
            }
        } else {
            Err(FcmError::InvalidAndroidRegistrationResponse.into())
        }
    }

    async fn send_raw(&self, client: &HyperClient, body: Bytes) -> Result<Bytes> {
        let auth = format!("AidLogin {}:{}", self.android_id, self.security_token);

        let request = http::Request::builder()
            .uri(URI)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(header::AUTHORIZATION, auth)
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
