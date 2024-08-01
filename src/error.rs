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
    /// Couldn't serialize the JSON structure into a string.
    RequestSerialize(serde_json::Error),
    /// Couldn't deserialize the JSON string from the response.
    ResponseDeserialize(serde_json::Error),
    /// Couldn't convert the `Content-Length` header in the response to a string.
    ResponseLengthToStr(hyper::header::ToStrError),
    /// Couldn't find the `Content-Length` header in the response.
    ResponseNoLength,
    /// Couldn't parse the `Content-Length` header in the response.
    ResponseParseLength(std::num::ParseIntError),
    /// Couldn't receive the response.
    ResponseReceive(hyper::Error),
    /// Couldn't write the response into buffer.
    ResponseWrite(std::io::Error),
    /// Couldn't determine the default settings for the TLS protocol.
    TlsConnector(native_tls::Error),
    /// Couldn't handshake with the server.
    /// This error can be caused because the server has an invalid certificate.
    TlsHandshake(native_tls::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Connect(_) => write!(f, "connect failed"),
            Error::Handshake(_) => write!(f, "handshake failed"),
            Error::Http(_) => write!(f, "erroneous HTTP status-code received"),
            Error::RequestBuilder(_) => write!(f, "request failed to build"),
            Error::RequestSend(_) => write!(f, "error sending request"),
            Error::RequestSerialize(_) => write!(f, "error serializing request JSON"),
            Error::ResponseDeserialize(_) => write!(f, "error deserializing response JSON"),
            Error::ResponseLengthToStr(_) => write!(f, "invalid bytes in response content-length"),
            Error::ResponseNoLength => write!(f, "no response content-length"),
            Error::ResponseParseLength(_) => write!(f, "invalid response content-length"),
            Error::ResponseReceive(_) => write!(f, "error receiving response"),
            Error::ResponseWrite(_) => write!(f, "error writing response into buffer"),
            Error::TlsConnector(_) => write!(f, "TLS connector failed"),
            Error::TlsHandshake(_) => write!(f, "TLS handshake failed"),
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
            Error::RequestSerialize(err) => Some(err),
            Error::ResponseDeserialize(err) => Some(err),
            Error::ResponseLengthToStr(err) => Some(err),
            Error::ResponseParseLength(err) => Some(err),
            Error::ResponseReceive(err) => Some(err),
            Error::ResponseWrite(err) => Some(err),
            Error::TlsConnector(err) => Some(err),
            Error::TlsHandshake(err) => Some(err),
            _ => None,
        }
    }
}
