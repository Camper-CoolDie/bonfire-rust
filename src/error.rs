use hyper::http::Error as HttpError;
use hyper::Error as HyperError;
use json::JsonError;
use native_tls::Error as TlsError;
use std::io::Error as IoError;
use std::str::Utf8Error;

use hyper::StatusCode;

/// Result type returned from methods that can have `Error`s.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur while operating on a session.
///
/// # Source
///
/// An `Error` can be the result of connecting or requesting.
/// It may be caused by implementors of `Connector` or while processing `Session.request`.
#[derive(Debug)]
pub enum Error {
    /// Couldn't connect to the server.
    Connect(IoError),
    /// Couldn't handshake with the server.
    Handshake(HyperError),
    /// The server returned an erroneous HTTP status code.
    ///
    /// * `413`: A `RequestKind::Standart` request was sent to the Bonfire server.
    /// * `429`: Too many requests were sent.
    /// * `500`: Internal server error. The request may have errors.
    Http(StatusCode),
    /// Couldn't build a request.
    /// This error can be caused because of an invalid parameter, such as `endpoint`.
    RequestBuilder(HttpError),
    /// Couldn't send the request.
    RequestSend(HyperError),
    /// Couldn't parse the JSON string from the response.
    ResponseParseJson(JsonError),
    /// Couldn't receive the response.
    ResponseReceive(HyperError),
    /// Couldn't convert the response to a valid UTF-8 string.
    ResponseUtf8(Utf8Error),
    /// Couldn't determine the default settings for the TLS protocol.
    TlsConnector(TlsError),
    /// Couldn't handshake with the server.
    /// This error can be caused because the server has an invalid certificate.
    TlsHandshake(TlsError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Connect(err) => write!(f, "{}", err),
            Error::Handshake(err) => write!(f, "{}", err),
            Error::Http(status) => write!(f, "erroneous HTTP status-code received: {}", status),
            Error::RequestBuilder(err) => write!(f, "{}", err),
            Error::RequestSend(err) => write!(f, "{}", err),
            Error::ResponseParseJson(err) => write!(f, "{}", err),
            Error::ResponseReceive(err) => write!(f, "{}", err),
            Error::ResponseUtf8(err) => write!(f, "{}", err),
            Error::TlsConnector(err) => write!(f, "{}", err),
            Error::TlsHandshake(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Connect(err) => Some(err),
            Error::Handshake(err) => Some(err),
            Error::RequestBuilder(err) => Some(err),
            Error::RequestSend(err) => Some(err),
            Error::ResponseParseJson(err) => Some(err),
            Error::ResponseReceive(err) => Some(err),
            Error::ResponseUtf8(err) => Some(err),
            Error::TlsConnector(err) => Some(err),
            Error::TlsHandshake(err) => Some(err),
            _ => None,
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::Connect(err)
    }
}

impl From<HttpError> for Error {
    fn from(err: HttpError) -> Self {
        Error::RequestBuilder(err)
    }
}

impl From<JsonError> for Error {
    fn from(err: JsonError) -> Self {
        Error::ResponseParseJson(err)
    }
}

impl From<Utf8Error> for Error {
    fn from(err: Utf8Error) -> Self {
        Error::ResponseUtf8(err)
    }
}
