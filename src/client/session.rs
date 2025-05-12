use super::{Error, Result};
use crate::{Connector, ConnectorWrapper};
use http::uri::PathAndQuery;
use http::{header, HeaderMap, HeaderValue, Method, Request, Uri};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::client::conn::http1::SendRequest;
use std::io::Write;

pub(super) struct Session {
    host: String,
    path_and_query: String,
    sender: SendRequest<Full<Bytes>>,
}
impl Session {
    pub(super) async fn connect(uri: &Uri) -> Result<Self> {
        let connector = ConnectorWrapper::new(uri)?;

        Ok(Self {
            host: uri.host().unwrap().to_owned(),
            path_and_query: uri
                .path_and_query()
                .unwrap_or(&PathAndQuery::from_static("/"))
                .as_str()
                .to_owned(),
            sender: connector.connect().await?,
        })
    }

    pub(super) async fn send_raw(
        &mut self,
        body: Bytes,
        headers: &HeaderMap<HeaderValue>,
    ) -> Result<Bytes> {
        let mut builder = Request::builder()
            .uri(&self.path_and_query)
            .method(Method::POST)
            .header(header::HOST, &self.host);
        for (key, value) in headers {
            builder = builder.header(key, value)
        }

        let request = builder.body(Full::new(body))?;
        let mut response = self.sender.send_request(request).await?;

        let status = response.status();
        if status.is_success() {
            let length = response
                .headers()
                .get(header::CONTENT_LENGTH)
                .ok_or(Error::ResponseMissingLength)?
                .to_str()?
                .parse()?;

            let mut body = Vec::with_capacity(length);
            while let Some(next) = response.frame().await {
                let frame = next?;
                if let Some(chunk) = frame.data_ref() {
                    body.write_all(chunk)?;
                }
            }

            Ok(Bytes::from(body))
        } else {
            Err(Error::UnsuccessfulResponse(status))
        }
    }
}
