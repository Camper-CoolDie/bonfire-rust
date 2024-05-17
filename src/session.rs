use crate::builder::Builder;
use crate::connector::Connector;
use crate::error::Error;
use crate::request_kind::RequestKind;
use crate::result::Result;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::client::conn::http1::SendRequest;
use hyper::{header, Method, Request};
use json::JsonValue;

/// Represents a session with the server. A session can be used to send continuous requests.
pub struct Session {
    sender: SendRequest<Full<Bytes>>,
    host: String,
    authorization: String,
    kind: RequestKind,
}
impl Session {
    /// Connect to the server.
    ///
    /// ```no_run
    /// # use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    /// # use bonfire::session::*;
    /// #
    /// # let host = "localhost";
    /// # let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// # tokio_test::block_on(async {
    /// #
    /// let mut session = Session::connect(
    ///     &Builder::new(),
    ///     SecureConnector::new(host, addr)
    /// ).await?;
    /// #
    /// # Ok::<(), Error>(()) }).unwrap();
    /// ```
    pub async fn connect<C>(builder: &Builder, connector: C) -> Result<Self>
    where
        C: Connector,
    {
        let sender = connector.connect().await?;

        Ok(Self {
            sender,
            host: connector.host(),
            authorization: builder.authorization.clone(),
            kind: builder.kind.clone(),
        })
    }

    /// Send a request.
    ///
    /// ```no_run
    /// # use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    /// # use bonfire::session::*;
    /// #
    /// # let host = "localhost";
    /// # let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// # let endpoint = "/";
    /// # let object = json::object!{};
    /// # tokio_test::block_on(async {
    /// #
    /// # let mut session = Session::builder()
    /// #     .connect(SecureConnector::new(host, addr)).await?;
    /// let response = session.request(endpoint, object).await?;
    /// #
    /// # Ok::<(), Error>(()) }).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// * `Error::RequestBuilder`
    /// * `Error::RequestSend`
    /// * `Error::ResponseReceive`
    /// * `Error::ResponseUtf8`
    /// * `Error::ResponseParseJson`
    /// * `Error::Http`
    pub async fn request<T>(&mut self, endpoint: T, json: JsonValue) -> Result<JsonValue>
    where
        String: From<T>,
    {
        let endpoint: String = endpoint.into();
        let user_agent: String =
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        let body = json::stringify(json);
        let body = body.into_bytes();
        let body = match self.kind {
            RequestKind::Standart => body,
            RequestKind::Bonfire => [&(body.len() as u32).to_be_bytes(), &body as &[u8]].concat(),
        };

        let request = Request::builder()
            .uri(endpoint)
            .method(Method::POST)
            .header(header::HOST, self.host.clone())
            .header(header::CONNECTION, "keep-alive")
            .header(header::USER_AGENT, user_agent)
            .header(header::AUTHORIZATION, self.authorization.clone())
            .body(Full::new(Bytes::from(body)))
            .map_err(Error::RequestBuilder)?;

        let mut response = self
            .sender
            .send_request(request)
            .await
            .map_err(Error::RequestSend)?;

        let status = response.status();
        if status.is_success() {
            let mut string = String::new();
            while let Some(next) = response.frame().await {
                let frame = next.map_err(Error::ResponseReceive)?;
                if let Some(chunk) = frame.data_ref() {
                    string.push_str(std::str::from_utf8(chunk).map_err(Error::ResponseUtf8)?);
                }
            }

            let body = json::parse(&string).map_err(Error::ResponseParseJson)?;
            Ok(body)
        } else {
            Err(Error::Http(status))
        }
    }

    /// Create a new `Builder` to build a new `Session`.
    ///
    /// ```no_run
    /// # use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    /// # use bonfire::session::*;
    /// #
    /// # let host = "localhost";
    /// # let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// # tokio_test::block_on(async {
    /// #
    /// let mut session = Session::builder()
    ///     /* ... */
    ///     .connect(SecureConnector::new(host, addr)).await?;
    /// #
    /// # Ok::<(), Error>(()) }).unwrap();
    /// ```
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}
