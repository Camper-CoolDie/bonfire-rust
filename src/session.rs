use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::client::conn::http1::{handshake, SendRequest};
use hyper::{header, Method, Request, StatusCode};
use hyper_util::rt::TokioIo;
use json::JsonValue;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::task;
use tokio_native_tls::TlsConnector;

/// Represents errors that can occur while operating on a session.
///
/// # Source
///
/// An `Error` can be the result of connecting or requesting.
/// It may be caused by implementors of `Connector` or while processing `Session.request`.
#[derive(Debug)]
pub enum Error {
    /// Couldn't connect to the server.
    Connect(std::io::Error),
    /// Couldn't handshake with the server.
    Handshake(hyper::Error),
    /// The server returned an erroneous HTTP status code.
    ///
    /// * `413`: A `RequestKind::Standart` request was sent to the Bonfire server.
    /// * `429`: Too many requests were sent.
    /// * `500`: Internal server error. The request may have errors.
    Http(StatusCode),
    /// Couldn't build a request.
    /// This error can be caused because of an invalid parameter, such as `endpoint`.
    RequestBuilder(hyper::http::Error),
    /// Couldn't send the request.
    RequestSend(hyper::Error),
    /// Couldn't parse the JSON string from the response.
    ResponseParseJson(json::JsonError),
    /// Couldn't receive the response.
    ResponseReceive(hyper::Error),
    /// Couldn't convert the response to a valid UTF-8 string.
    ResponseUtf8(std::str::Utf8Error),
    /// Couldn't determine the default settings for the TLS protocol.
    TlsConnector(native_tls::Error),
    /// Couldn't handshake with the server.
    /// This error can be caused because the server has an invalid certificate.
    TlsHandshake(native_tls::Error),
}

/// Result type returned from methods that can have `Error`s.
pub type Result<T> = std::result::Result<T, Error>;

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

/// A session builder. A `Builder` can be used to construct a `Session` through a builder-like pattern.
pub struct Builder {
    authorization: String,
    kind: RequestKind,
}
impl Builder {
    /// Create a new `Builder` with default arguments.
    ///
    /// ```
    /// # use bonfire::session::*;
    /// #
    /// let mut builder: &mut Builder = &mut Builder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            authorization: String::new(),
            kind: RequestKind::default(),
        }
    }

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
    /// let mut session = Builder::new()
    ///     /* ... */
    ///     .connect(SecureConnector::new(host, addr)).await?;
    /// #
    /// # Ok::<(), Error>(()) }).unwrap();
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
    /// # use bonfire::session::*;
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
    /// # use bonfire::session::*;
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
    fn default() -> Self {
        Builder::new()
    }
}

/// Trait representing a connector.
pub trait Connector {
    #[doc(hidden)]
    fn connect(&self)
        -> impl std::future::Future<Output = Result<SendRequest<Full<Bytes>>>> + Send;

    #[doc(hidden)]
    fn host(&self) -> String;
}

/// Represents an insecure connector for connecting without the TLS protocol (similar to "http://").
///
/// May cause the following errors when connecting:
///
/// * `Error::Connect`
/// * `Error::Handshake`
pub struct InsecureConnector {
    host: String,
    addr: SocketAddr,
}
impl InsecureConnector {
    /// Create a new `InsecureConnector`.
    ///
    /// ```
    /// # use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    /// # use bonfire::session::*;
    /// #
    /// # let host = "localhost";
    /// # let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// let connector = InsecureConnector::new(host, addr);
    /// ```
    pub fn new<T>(host: T, addr: SocketAddr) -> Self
    where
        String: From<T>,
    {
        Self {
            host: host.into(),
            addr,
        }
    }
}

impl Connector for InsecureConnector {
    async fn connect(&self) -> Result<SendRequest<Full<Bytes>>> {
        let stream = TcpStream::connect(self.addr)
            .await
            .map_err(Error::Connect)?;
        let io = TokioIo::new(stream);
        let (sender, conn) = handshake(io).await.map_err(Error::Handshake)?;
        task::spawn(conn);

        Ok(sender)
    }

    fn host(&self) -> String {
        self.host.clone()
    }
}

/// Represents a secure connector for connecting using the TLS protocol (similar to "https://").
///
/// May cause the following errors when connecting:
///
/// * `Error::Connect`
/// * `Error::Handshake`
/// * `Error::TlsConnector`
/// * `Error::TlsHandshake`
pub struct SecureConnector {
    host: String,
    addr: SocketAddr,
}
impl SecureConnector {
    /// Create a new `SecureConnector`.
    ///
    /// ```
    /// # use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    /// # use bonfire::session::*;
    /// #
    /// # let host = "localhost";
    /// # let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// let connector = SecureConnector::new(host, addr);
    /// ```
    pub fn new<T>(host: T, addr: SocketAddr) -> Self
    where
        String: From<T>,
    {
        Self {
            host: host.into(),
            addr,
        }
    }
}

impl Connector for SecureConnector {
    async fn connect(&self) -> Result<SendRequest<Full<Bytes>>> {
        let stream = TcpStream::connect(self.addr)
            .await
            .map_err(Error::Connect)?;
        let connector = native_tls::TlsConnector::new().map_err(Error::TlsConnector)?;
        let connector = TlsConnector::from(connector);
        let stream = connector
            .connect(&self.host.clone(), stream)
            .await
            .map_err(Error::TlsHandshake)?;

        let io = TokioIo::new(stream);
        let (sender, conn) = handshake(io).await.map_err(Error::Handshake)?;
        task::spawn(conn);

        Ok(sender)
    }

    fn host(&self) -> String {
        self.host.clone()
    }
}
