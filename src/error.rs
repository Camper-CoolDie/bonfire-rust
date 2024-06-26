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
    Connect(std::io::Error),
    /// Couldn't handshake with the server.
    Handshake(hyper::Error),
    /// The server returned an erroneous HTTP status code.
    ///
    /// * `413`: A `RequestKind::Standart` request was sent to the Bonfire server.
    /// * `429`: Too many requests were sent.
    /// * `500`: Internal server error. The request may have errors.
    Http(StatusCode),
    /// Couldn't build a request.
    /// This error can be caused because of an invalid parameter, such as `endpoint`.
    RequestBuilder(hyper::http::Error),
    /// Couldn't send the request.
    RequestSend(hyper::Error),
    /// Couldn't parse the JSON string from the response.
    ResponseParseJson(json::JsonError),
    /// Couldn't receive the response.
    ResponseReceive(hyper::Error),
    /// Couldn't convert the response to a valid UTF-8 string.
    ResponseUtf8(std::str::Utf8Error),
    /// Couldn't determine the default settings for the TLS protocol.
    TlsConnector(native_tls::Error),
    /// Couldn't handshake with the server.
    /// This error can be caused because the server has an invalid certificate.
    TlsHandshake(native_tls::Error),
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

impl std::error::Error for Error {}
