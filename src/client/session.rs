use bytes::{BufMut, Bytes, BytesMut};
use http::{header, HeaderMap, HeaderValue, Method, Uri};
use http_body_util::{BodyExt, Full};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::{MeliorError, MeliorResponse, Query, Request, RootError, RootResponse};
use crate::{Error, Result};

static USER_AGENT: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/{} ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        os_info::get()
    )
});

pub(super) struct Session {
    hyper_client: HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>,
    uri: Uri,
}
impl Session {
    pub(super) fn new(uri: &Uri) -> Self {
        let connector = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_or_http()
            .enable_all_versions()
            .build();

        Self {
            hyper_client: HyperClient::builder(TokioExecutor::new()).build(connector),
            uri: uri.clone(),
        }
    }

    pub(super) async fn send_request<'a, R: Serialize, S: DeserializeOwned>(
        &self,
        request: Request<'a, R>,
        attachments: Vec<&[u8]>,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<S> {
        let json_body = serde_json::to_vec(&request)?;

        let data_length = attachments.iter().map(|slice| slice.len()).sum::<usize>();

        // 4 bytes for u32 length
        let mut payload = BytesMut::with_capacity(4 + json_body.len() + data_length);
        payload.put_u32(json_body.len() as u32);
        payload.put_slice(&json_body);
        for attachment in attachments.into_iter() {
            payload.put_slice(attachment);
        }

        let response = self.send_raw(payload.freeze(), &headers, None).await?;

        match serde_json::from_slice::<RootResponse<S>>(&response)? {
            RootResponse::Ok(content) => Ok(content),
            RootResponse::Error(error) => Err(RootError::try_from(error)
                .inspect_err(|error| tracing::error!(?error, "failed to parse a root error"))
                .map_or_else(|error| error, Error::from)),
        }
    }

    pub(super) async fn send_query<R: Serialize, S: DeserializeOwned>(
        &self,
        query: Query<R>,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<S> {
        let body = serde_json::to_vec(&query)?;

        let response = self
            .send_raw(Bytes::from(body), &headers, Some("application/json"))
            .await?;

        let response = serde_json::from_slice::<MeliorResponse<S>>(&response)?;
        response.data.ok_or(
            response
                .errors
                .and_then(|errors| {
                    errors
                        .into_iter()
                        .next()
                        .map(MeliorError::from)
                        .map(Error::from)
                })
                .unwrap_or(Error::InvalidMeliorResponse),
        )
    }

    async fn send_raw(
        &self,
        body: Bytes,
        headers: &HeaderMap<HeaderValue>,
        body_kind: Option<&'static str>,
    ) -> Result<Bytes> {
        let mut builder = http::Request::builder()
            .uri(&self.uri)
            .method(Method::POST)
            .header(header::USER_AGENT, &**USER_AGENT);

        if let Some(kind) = body_kind {
            builder = builder.header(header::CONTENT_TYPE, kind);
        }
        for (key, value) in headers {
            builder = builder.header(key, value);
        }

        let request = builder.body(Full::new(body))?;
        let response = self.hyper_client.request(request).await?;

        let status = response.status();
        status
            .is_success()
            .then_some(response.collect().await?.to_bytes())
            .ok_or(Error::UnsuccessfulResponse(status))
    }
}
