use crate::{Error, Result};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::{handshake, SendRequest};
use hyper_util::rt::TokioIo;
use tokio::net::{TcpStream, ToSocketAddrs};
use tokio::task;
use tokio_native_tls::TlsConnector;

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
pub struct InsecureConnector<A> {
    host: String,
    addrs: A,
}
impl<A: ToSocketAddrs + Clone + Send + Sync> InsecureConnector<A> {
    /// Create a new `InsecureConnector`.
    ///
    /// ```
    /// # use bonfire::InsecureConnector;
    /// #
    /// # let host = "localhost";
    /// # let addr = (host, 8080);
    /// let connector = InsecureConnector::new(host, addr);
    /// ```
    pub fn new<T>(host: T, addrs: A) -> Self
    where
        String: From<T>,
    {
        Self {
            host: host.into(),
            addrs,
        }
    }
}

impl<A: ToSocketAddrs + Clone + Send + Sync> Connector for InsecureConnector<A> {
    async fn connect(&self) -> Result<SendRequest<Full<Bytes>>> {
        let stream = TcpStream::connect(self.addrs.clone())
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
pub struct SecureConnector<A> {
    host: String,
    addrs: A,
}
impl<A: ToSocketAddrs + Clone + Send + Sync> SecureConnector<A> {
    /// Create a new `SecureConnector`.
    ///
    /// ```
    /// # use bonfire::SecureConnector;
    /// #
    /// # let host = "localhost";
    /// # let addr = (host, 8080);
    /// let connector = SecureConnector::new(host, addr);
    /// ```
    pub fn new<T>(host: T, addrs: A) -> Self
    where
        String: From<T>,
    {
        Self {
            host: host.into(),
            addrs,
        }
    }
}

impl<A: ToSocketAddrs + Clone + Send + Sync> Connector for SecureConnector<A> {
    async fn connect(&self) -> Result<SendRequest<Full<Bytes>>> {
        let stream = TcpStream::connect(self.addrs.clone())
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
