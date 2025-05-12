mod insecure;
mod secure;

use http::uri::Scheme;
use http::Uri;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::SendRequest;
use insecure::InsecureConnector;
use secure::SecureConnector;
use std::error::Error as StdError;
use std::fmt;

pub(crate) trait Connector {
    fn connect(&self)
        -> impl std::future::Future<Output = Result<SendRequest<Full<Bytes>>>> + Send;
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    EmptyHost,
    EmptyScheme,
    HyperError(hyper::Error),
    IoError(std::io::Error),
    TlsError(native_tls::Error),
    UnsupportedScheme(Scheme),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyHost => write!(f, "URI host is empty"),
            Self::EmptyScheme => write!(f, "URI scheme is empty"),
            Self::HyperError(_) => write!(f, "hyper error"),
            Self::IoError(_) => write!(f, "IO error"),
            Self::TlsError(_) => write!(f, "TLS error"),
            Self::UnsupportedScheme(scheme) => {
                write!(f, "URI scheme is unsupported: {}", scheme)
            }
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match *self {
            Self::HyperError(ref e) => Some(e),
            Self::IoError(ref e) => Some(e),
            Self::TlsError(ref e) => Some(e),
            _ => None,
        }
    }
}

impl From<hyper::Error> for Error {
    fn from(e: hyper::Error) -> Self {
        Self::HyperError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<native_tls::Error> for Error {
    fn from(e: native_tls::Error) -> Self {
        Self::TlsError(e)
    }
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
