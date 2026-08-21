use std::collections::HashMap;

use bytes::Bytes;
use http::{Method, header};
use http_body_util::{BodyExt as _, Either, Full};
use serde::Deserialize;
use uuid::Uuid;

use crate::{Error, HyperClient, Result};

const URI: &str = "https://android.clients.google.com/c2dm/register3";

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum RegistrationResponse {
    Ok {
        #[serde(rename = "token")]
        gcm_token: String,
    },
    Error {
        #[serde(rename = "Error")]
        code: String,
    },
}

pub(crate) struct RegistrationRequest<'a> {
    vapid_key: &'a str,
    uri: &'a str,
    android_id: u64,
    security_token: u64,
}
impl<'a> RegistrationRequest<'a> {
    pub(crate) fn new(
        vapid_key: &'a str,
        uri: &'a str,
        android_id: u64,
        security_token: u64,
    ) -> Self {
        Self {
            vapid_key,
            uri,
            android_id,
            security_token,
        }
    }

    pub(crate) async fn send_request(&self, client: &HyperClient) -> Result<RegistrationResponse> {
        let subtype = format!("wp:{}/#{}", self.uri, Uuid::new_v4().hyphenated());
        let android_id = self.android_id.to_string();

        let mut params = HashMap::with_capacity(4);
        params.insert("app", "com.chrome.linux");
        params.insert("X-subtype", &subtype);
        params.insert("device", &android_id);
        params.insert("sender", self.vapid_key);
        let payload = serde_urlencoded::to_string(&params)?;

        let bytes = self.send_raw(client, Bytes::from(payload)).await?;
        let response = serde_urlencoded::from_bytes(&bytes)?;
        Ok(response)
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
