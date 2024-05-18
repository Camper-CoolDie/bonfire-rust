use crate::result::Result;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::client::conn::http1::SendRequest;

/// Trait representing a connector.
pub trait Connector {
    #[doc(hidden)]
    fn connect(&self)
        -> impl std::future::Future<Output = Result<SendRequest<Full<Bytes>>>> + Send;

    #[doc(hidden)]
    fn host(&self) -> String;
}
