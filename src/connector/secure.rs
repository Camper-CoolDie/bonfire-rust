use super::{Connector, Error, Result};
use http::Uri;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::{handshake, SendRequest};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio::task;

pub(crate) struct SecureConnector {
    host: String,
    port: u16,
}
impl SecureConnector {
    pub(crate) fn new(uri: &Uri) -> Result<Self> {
        if uri.scheme().is_some_and(|scheme| scheme != "https") {
            Err(Error::UnsupportedScheme(uri.scheme().unwrap().clone()))?;
        }

        let host = uri.host().ok_or(Error::EmptyHost)?.to_owned();
        let port = uri.port_u16().unwrap_or(443);

        Ok(Self { host, port })
    }
}

impl Connector for SecureConnector {
    async fn connect(&self) -> Result<SendRequest<Full<Bytes>>> {
        let stream = TcpStream::connect((self.host.as_str(), self.port)).await?;
        let connector = native_tls::TlsConnector::new()?;
        let connector = tokio_native_tls::TlsConnector::from(connector);
        let stream = connector.connect(&self.host, stream).await?;
        let io = TokioIo::new(stream);
        let (sender, conn) = handshake(io).await?;
        task::spawn(conn);

        Ok(sender)
    }
}
