use crate::{Error, Result};
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::{handshake, SendRequest};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpStream;
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
pub struct InsecureConnector {
    host: String,
    addr: SocketAddr,
}
impl InsecureConnector {
    /// Create a new `InsecureConnector`.
    ///
    /// ```
    /// # use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    /// # use bonfire::InsecureConnector;
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
    /// # use bonfire::SecureConnector;
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
