use std::sync::Arc;

use http::Uri;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::{handshake, SendRequest};
use hyper_util::rt::TokioIo;
use rustls_pki_types::ServerName;
use tokio::net::TcpStream;
use tokio::task;
use tokio_rustls::TlsConnector;

use super::{Connector, Error, Result};

pub(crate) struct SecureConnector {
    host: String,
    port: u16,
    config: Arc<rustls::ClientConfig>,
}

impl SecureConnector {
    pub(crate) fn new(uri: &Uri) -> Result<Self> {
        if uri.scheme().is_some_and(|scheme| scheme != "https") {
            Err(Error::UnsupportedScheme(uri.scheme().unwrap().clone()))?;
        }

        let host = uri.host().ok_or(Error::EmptyHost)?.to_owned();
        let port = uri.port_u16().unwrap_or(443);

        let mut root_cert_store = rustls::RootCertStore::empty();
        root_cert_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

        let config = rustls::ClientConfig::builder()
            .with_root_certificates(root_cert_store)
            .with_no_client_auth();

        Ok(Self {
            host,
            port,
            config: Arc::new(config),
        })
    }
}

impl Connector for SecureConnector {
    async fn connect(&self) -> Result<SendRequest<Full<Bytes>>> {
        let stream = TcpStream::connect((self.host.as_str(), self.port)).await?;
        let connector = TlsConnector::from(self.config.clone());
        let server_name = ServerName::try_from(self.host.as_str())
            .map_err(|name| Error::InvalidDnsName(name.to_string()))?
            .to_owned();
        let stream = connector.connect(server_name, stream).await?;
        let io = TokioIo::new(stream);
        let (sender, conn) = handshake(io).await?;
        task::spawn(conn);

        Ok(sender)
    }
}
