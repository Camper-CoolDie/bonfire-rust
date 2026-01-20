use std::result::Result as StdResult;

use http::StatusCode;
use thiserror::Error;

use crate::client::JwtError;
use crate::models::auth;
use crate::{MeliorError, RootError};

/// A type alias for [`Result<T, Error>`][StdResult<T, Error>].
pub type Result<T> = StdResult<T, Error>;

/// Represents errors that can occur while operating with a client.
///
/// # Source
///
/// An `Error` can be the result of operations like constructing a request or parsing a response.
#[derive(Error, Debug)]
pub enum Error {
    /// The provided attachment weights more the server can process
    #[error("attachment is too large")]
    AttachmentTooLarge,
    /// Can't authenticate
    #[error("authentication error")]
    AuthError(#[from] auth::Error),
    /// Can't parse authentication credentials
    #[error("JWT error")]
    JwtError(#[from] JwtError),
    /// Can't (de)serialize a JSON object
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
    /// Can't build a request
    #[error("HTTP error")]
    HttpError(#[from] http::Error),
    /// Can't send data to the server
    #[error("hyper client error")]
    HyperClientError(#[from] hyper_util::client::legacy::Error),
    /// Can't receive data from the server
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    /// The melior server returned neither data nor an error
    #[error("invalid melior response")]
    InvalidMeliorResponse,
    /// The melior server returned an error
    #[error("melior server error")]
    MeliorError(#[from] MeliorError),
    /// The constructed request weights more than the server can process
    #[error("request is too large")]
    RequestTooLarge,
    /// The root server returned an error
    #[error("root server error")]
    RootError(#[from] RootError),
    /// The server returned an unsuccessful HTTP status code. Some of the most common codes are:
    ///
    /// * `429`: Too many requests in a short period of time
    /// * `500`: The server couldn't parse the request
    /// * `502`: The server is down
    #[error(
        "unsuccessful response: {}{}",
        .0.as_u16(),
        .0.canonical_reason().map_or(String::new(), |reason| " ".to_owned() + reason)
    )]
    UnsuccessfulResponse(StatusCode),
}
