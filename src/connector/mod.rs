mod insecure;
mod secure;

use http::uri::Scheme;
use http::Uri;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::SendRequest;
use insecure::InsecureConnector;
use secure::SecureConnector;
use thiserror::Error;

pub(crate) trait Connector {
    fn connect(&self)
        -> impl std::future::Future<Output = Result<SendRequest<Full<Bytes>>>> + Send;
}
pub(crate) enum ConnectorWrapper {
    Insecure(InsecureConnector),
    Secure(SecureConnector),
}
impl ConnectorWrapper {
    pub(crate) fn new(uri: &Uri) -> Result<Self> {
        let scheme = uri.scheme().ok_or(Error::EmptyScheme)?;

        match scheme.as_str() {
            "http" => Ok(Self::Insecure(InsecureConnector::new(uri)?)),
            "https" => Ok(Self::Secure(SecureConnector::new(uri)?)),
            &_ => Err(Error::UnsupportedScheme(scheme.clone())),
        }
    }
}

impl Connector for ConnectorWrapper {
    async fn connect(&self) -> Result<SendRequest<Full<Bytes>>> {
        match self {
            Self::Insecure(connector) => connector.connect().await,
            Self::Secure(connector) => connector.connect().await,
        }
    }
}

/// A type alias for [Result<T, connector::Error>][std::result::Result<T, Error>].
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur while connecting to the servers.
#[derive(Error, Debug)]
pub enum Error {
    /// URI host is not specified
    #[error("unspecified URI host")]
    EmptyHost,
    /// URI scheme is not specified
    #[error("unspecified URI scheme")]
    EmptyScheme,
    /// Can't handshake with the server
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    /// The provided host is not a valid DNS name
    #[error("invalid DNS name: {0}")]
    InvalidDnsName(String),
    /// Can't create a TCP connection
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    /// Can't determine the settings for connecting
    #[error("TLS error")]
    TlsError(#[from] rustls::Error),
    /// URI scheme is not supported. It must be either `http` or `https`
    #[error("unsupported URI scheme {0:?}")]
    UnsupportedScheme(Scheme),
}
