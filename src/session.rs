use crate::{Connector, Error, Result};
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::client::conn::http1::SendRequest;
use hyper::{header, Method, Request};
use serde_json::Value;
use std::io::Write;

/// Represents the type of a request.
#[derive(Clone, Default)]
pub enum RequestKind {
    /// The request body will have only the JSON string.
    #[default]
    Standart,
    /// The request body will have the body length put before the JSON string.
    /// Useful when sending a request to the Bonfire server.
    Bonfire,
}

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
    /// # use bonfire::session::Builder;
    /// # use bonfire::{Error, SecureConnector, Session};
    /// #
    /// # let host = "localhost";
    /// # let addr = (host, 8080);
    /// # tokio_test::block_on(async {
    /// let mut session = Session::connect(&Builder::new(), SecureConnector::new(host, addr)).await?;
    /// #
    /// #     Ok::<(), Error>(())
    /// # })
    /// # .unwrap();
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
    /// # use bonfire::{Error, SecureConnector, Session};
    /// # use serde_json::json;
    /// #
    /// # let host = "localhost";
    /// # let addr = (host, 8080);
    /// # let endpoint = "/";
    /// # let json = json!({});
    /// # tokio_test::block_on(async {
    /// #     let mut session = Session::builder()
    /// #         .connect(SecureConnector::new(host, addr))
    /// #         .await?;
    /// let response = session.request(endpoint, json).await?;
    /// #
    /// #     Ok::<(), Error>(())
    /// # })
    /// # .unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// * `Error::RequestBuilder`
    /// * `Error::RequestSend`
    /// * `Error::RequestSerialize`
    /// * `Error::ResponseDeserialize`
    /// * `Error::ResponseReceive`
    /// * `Error::ResponseWrite`
    /// * `Error::Http`
    pub async fn request<T>(&mut self, endpoint: T, json: Value) -> Result<Value>
    where
        String: From<T>,
    {
        let endpoint: String = endpoint.into();
        let user_agent: String =
            format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        let mut body: Vec<u8> = Vec::new();
        let _ = serde_json::to_writer(&mut body, &json).map_err(Error::RequestSerialize);
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
            let length = response
                .headers()
                .get(header::CONTENT_LENGTH)
                .ok_or(Error::ResponseNoLength)?;
            let length = length.to_str().map_err(Error::ResponseLengthToStr)?;
            let length: usize = length.parse().map_err(Error::ResponseParseLength)?;

            let mut body: Vec<u8> = Vec::with_capacity(length);
            while let Some(next) = response.frame().await {
                let frame = next.map_err(Error::ResponseReceive)?;
                if let Some(chunk) = frame.data_ref() {
                    body.write(chunk).map_err(Error::ResponseWrite)?;
                }
            }
            body.flush().map_err(Error::ResponseWrite)?;

            let body = serde_json::from_slice(&body).map_err(Error::ResponseDeserialize)?;
            Ok(body)
        } else {
            Err(Error::Http(status))
        }
    }

    /// Create a new `Builder` to build a new `Session`.
    ///
    /// ```no_run
    /// # use bonfire::{Error, SecureConnector, Session};
    /// #
    /// # let host = "localhost";
    /// # let addr = (host, 8080);
    /// # tokio_test::block_on(async {
    /// let mut session = Session::builder()
    ///     /* ... */
    ///     .connect(SecureConnector::new(host, addr))
    ///     .await?;
    /// #
    /// #     Ok::<(), Error>(())
    /// # })
    /// # .unwrap();
    /// ```
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

/// A session builder. A `Builder` can be used to construct a `Session` through a builder-like pattern.
pub struct Builder {
    authorization: String,
    kind: RequestKind,
}
impl Builder {
    /// Create a new `Builder` with default arguments.
    ///
    /// ```
    /// # use bonfire::session::Builder;
    /// #
    /// let mut builder: &mut Builder = &mut Builder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            authorization: String::default(),
            kind: RequestKind::default(),
        }
    }

    /// Connect to the server.
    ///
    /// ```no_run
    /// # use bonfire::session::Builder;
    /// # use bonfire::{Error, SecureConnector};
    /// #
    /// # let host = "localhost";
    /// # let addr = (host, 8080);
    /// # tokio_test::block_on(async {
    /// let mut session = Builder::new()
    ///     /* ... */
    ///     .connect(SecureConnector::new(host, addr))
    ///     .await?;
    /// #
    /// #     Ok::<(), Error>(())
    /// # })
    /// # .unwrap();
    /// ```
    pub async fn connect<C>(&self, connector: C) -> Result<Session>
    where
        C: Connector,
    {
        Session::connect(self, connector).await
    }

    /// Set the authorization header for this `Session`.
    ///
    /// ```
    /// # use bonfire::session::Builder;
    /// #
    /// # let mut builder: &mut Builder = &mut Builder::new();
    /// builder = builder.authorization("Bearer <token>");
    /// ```
    pub fn authorization<T>(&mut self, authorization: T) -> &mut Self
    where
        String: From<T>,
    {
        self.authorization = authorization.into();
        self
    }

    /// Set the type of this `Session`.
    ///
    /// ```
    /// # use bonfire::session::{Builder, RequestKind};
    /// #
    /// # let mut builder: &mut Builder = &mut Builder::new();
    /// builder = builder.kind(RequestKind::default());
    /// ```
    pub fn kind(&mut self, kind: RequestKind) -> &mut Self {
        self.kind = kind;
        self
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
