use bytes::{BufMut as _, Bytes, BytesMut};
use http::{header, HeaderMap, HeaderValue, Method, Uri};
use http_body_util::{BodyExt as _, Full};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::rt::TokioExecutor;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::client::service::USER_AGENT;
use crate::{Error, Result, RootError, RootRequest, RootResponse};

// The maximum allowed size for a request (10 MiB)
const PAYLOAD_MAX_LENGTH: usize = 10 * 1024 * 1024;

pub(crate) struct RootService {
    hyper_client: HyperClient<HttpsConnector<HttpConnector>, Full<Bytes>>,
    uri: Uri,
}
impl RootService {
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

    pub(crate) async fn send_request<R: Serialize, S: DeserializeOwned>(
        &self,
        request: RootRequest<'_, R>,
        attachments: Vec<&[u8]>,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<S> {
        let json_body = serde_json::to_vec(&request)?;

        let attachments_length = attachments.iter().map(|slice| slice.len()).sum::<usize>();
        let payload_length = 4 + json_body.len() + attachments_length;
        if payload_length > PAYLOAD_MAX_LENGTH {
            Err(Error::RequestTooLarge)?;
        }

        let mut payload = BytesMut::with_capacity(payload_length);
        // JSON length is already checked to not exceed PAYLOAD_MAX_LENGTH
        #[allow(clippy::cast_possible_truncation)]
        payload.put_u32(json_body.len() as u32);

        payload.put_slice(&json_body);
        for attachment in attachments {
            payload.put_slice(attachment);
        }

        let response = self.send_raw(payload.freeze(), &headers).await?;

        match serde_json::from_slice::<RootResponse<S>>(&response)? {
            RootResponse::Ok(content) => Ok(content),
            RootResponse::Error(error) => Err(RootError::try_from(error)
                .inspect_err(|error| tracing::error!(?error, "failed to parse root error"))
                .map_or_else(|error| error, Error::from)),
        }
    }

    async fn send_raw(&self, body: Bytes, headers: &HeaderMap<HeaderValue>) -> Result<Bytes> {
        let builder = http::Request::builder()
            .uri(&self.uri)
            .method(Method::POST)
            .header(header::USER_AGENT, &**USER_AGENT);

        let builder = headers
            .iter()
            .fold(builder, |builder, (key, value)| builder.header(key, value));

        let request = builder.body(Full::new(body))?;
        let response = self.hyper_client.request(request).await?;

        let status = response.status();
        if status.is_success() {
            Ok(response.collect().await?.to_bytes())
        } else {
            Err(Error::UnsuccessfulResponse(status))
        }
    }
}
