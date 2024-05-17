use crate::connector::Connector;
use crate::error::Error;
use crate::result::Result;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::handshake;
use hyper::client::conn::http1::SendRequest;
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::task;

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
