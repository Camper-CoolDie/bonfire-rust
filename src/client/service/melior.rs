use bytes::Bytes;
use http::{header, HeaderMap, HeaderValue, Method, Uri};
use http_body_util::{BodyExt, Full};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::client::service::USER_AGENT;
use crate::{Error, MeliorError, MeliorQuery, MeliorResponse, Result};

pub(crate) struct MeliorService {
    hyper_client: HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>,
    uri: Uri,
}
impl MeliorService {
    pub(crate) fn new(uri: &Uri) -> Self {
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

    pub(crate) async fn send_query<R: Serialize, S: DeserializeOwned>(
        &self,
        query: MeliorQuery<R>,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<S> {
        let body = serde_json::to_vec(&query)?;

        let bytes = self.send_raw(Bytes::from(body), &headers).await?;
        let response = serde_json::from_slice::<MeliorResponse<S>>(&bytes)?;

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

    async fn send_raw(&self, body: Bytes, headers: &HeaderMap<HeaderValue>) -> Result<Bytes> {
        let builder = http::Request::builder()
            .uri(&self.uri)
            .method(Method::POST)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, &**USER_AGENT);

        let builder = headers
            .iter()
            .fold(builder, |builder, (key, value)| builder.header(key, value));

        let request = builder.body(Full::new(body))?;
        let response = self.hyper_client.request(request).await?;

        let status = response.status();
        match status.is_success() {
            true => Ok(response.collect().await?.to_bytes()),
            false => Err(Error::UnsuccessfulResponse(status)),
        }
    }
}
