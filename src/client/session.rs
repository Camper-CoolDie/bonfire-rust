use bytes::{BufMut, Bytes, BytesMut};
use http::uri::PathAndQuery;
use http::{header, HeaderMap, HeaderValue, Method, Uri};
use http_body_util::{BodyExt, Full};
use hyper::client::conn::http1::SendRequest;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::models::{MeliorError, MeliorResponse, Query, Request, RootError, RootResponse};
use crate::{connector, Connector, ConnectorWrapper, Error, Result};

static USER_AGENT: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/{} ({})",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        os_info::get()
    )
});

pub(super) struct Session {
    sender: SendRequest<Full<Bytes>>,
    host: String,
    path_and_query: String,
}
impl Session {
    pub(super) async fn connect(uri: &Uri) -> connector::Result<Self> {
        // The 'Host' header must contain a port if provided
        let host = uri.host().ok_or(connector::Error::EmptyHost)?.to_owned()
            + &uri
                .port()
                .map_or(String::default(), |port| ":".to_owned() + port.as_str());

        let scheme = uri.scheme().ok_or(connector::Error::EmptyScheme)?;
        let path_and_query = uri
            .path_and_query()
            .unwrap_or(&PathAndQuery::from_static("/"))
            .as_str()
            .to_owned();

        tracing::info!(host, ?scheme, path_and_query, "connecting");
        let connector = ConnectorWrapper::new(uri)?;
        let sender = connector.connect().await.map_err(|error| {
            tracing::error!(?error, "failed to connect");
            error
        })?;

        Ok(Self {
            sender,
            host,
            path_and_query,
        })
    }

    pub(super) async fn send_request<'a, R: Serialize, S: DeserializeOwned>(
        &mut self,
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
            RootResponse::Error(error) => Err(match RootError::try_from(error) {
                Ok(root_error) => root_error.into(),
                Err(error) => {
                    tracing::error!(?error, "failed to parse a root error");
                    error
                }
            }),
        }
    }

    pub(super) async fn send_query<R: Serialize, S: DeserializeOwned>(
        &mut self,
        query: Query<R>,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<S> {
        let body = serde_json::to_vec(&query)?;

        let response = self
            .send_raw(Bytes::from(body), &headers, Some("application/json"))
            .await?;

        let response = serde_json::from_slice::<MeliorResponse<S>>(&response)?;
        if response
            .errors
            .as_ref()
            .is_some_and(|errors| !errors.is_empty())
        {
            // For simplicity we return only the first error
            Err(MeliorError::from(response.errors.unwrap().pop().unwrap()).into())
        } else {
            Ok(response
                .data
                .expect("melior response is missing both data and errors"))
        }
    }

    async fn send_raw(
        &mut self,
        body: Bytes,
        headers: &HeaderMap<HeaderValue>,
        body_kind: Option<&'static str>,
    ) -> Result<Bytes> {
        let mut builder = http::Request::builder()
            .uri(&self.path_and_query)
            .method(Method::POST)
            .header(header::HOST, &self.host)
            .header(header::USER_AGENT, &**USER_AGENT);

        if let Some(kind) = body_kind {
            builder = builder.header(header::CONTENT_TYPE, kind);
        }
        for (key, value) in headers {
            builder = builder.header(key, value);
        }

        let request = builder.body(Full::new(body))?;
        let response = self.sender.send_request(request).await?;

        let status = response.status();
        if status.is_success() {
            let body = response.collect().await?.to_bytes();
            Ok(body)
        } else {
            Err(Error::UnsuccessfulResponse(status))
        }
    }
}
