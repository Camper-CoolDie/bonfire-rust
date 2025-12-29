mod error;
mod insecure;
mod secure;

pub use error::{Error, Result};
use http::Uri;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::SendRequest;
use insecure::InsecureConnector;
use secure::SecureConnector;

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
