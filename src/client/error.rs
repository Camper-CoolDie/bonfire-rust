use std::error::Error as StdError;
use std::fmt::Debug;
use std::result::Result as StdResult;
use std::sync::Arc;

use http::StatusCode;
use thiserror::Error;

use crate::client::{JwtError, RequestError};
use crate::{MeliorError, RootError};

/// A type alias for [`Result<T, Error>`][StdResult].
pub type Result<T> = StdResult<T, Error>;

/// Represents errors that can occur while operating with the client.
///
/// # Source
///
/// An `Error` can be the result of operations such as constructing a request or parsing a
/// response.
#[derive(Error, Debug)]
pub enum Error {
    /// The client is already authenticated
    #[error("authenticated client")]
    AlreadyAuthenticated,
    /// The provided attachment exceeds the maximum size the server can process
    #[error("attachment is too large")]
    AttachmentTooLarge,
    /// An error occurred during JSON serialization or deserialization
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
    /// An error occurred while parsing authentication credentials.
    ///
    /// This error can also indicate that the client's internal authentication state management has
    /// encountered an invalid token (e.g., a malformed token received after a refresh). In this
    /// state, all subsequent requests (including logout) will fail with this error until a new
    /// session is established by calling [`Client::login()`][crate::Client::login()].
    #[error("JWT error")]
    JwtError(#[from] Arc<JwtError>),
    /// An HTTP-related error occurred during request construction
    #[error("HTTP error")]
    HttpError(#[from] http::Error),
    /// An error occurred within the Hyper client
    #[error("hyper client error")]
    HyperClientError(#[from] hyper_util::client::legacy::Error),
    /// A Hyper-related error occurred, typically during network communication
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    /// The Melior server returned a response with neither data nor a specific error
    #[error("invalid melior response")]
    InvalidMeliorResponse,
    /// The Melior server returned an error
    #[error("melior server error")]
    MeliorError(#[source] MeliorError),
    /// A request-specific error
    #[error("request-specific error")]
    RequestError(#[source] Box<dyn StdError + Send + Sync + 'static>),
    /// The constructed request exceeds the maximum size the server can process
    #[error("request is too large")]
    RequestTooLarge,
    /// The Root server returned an error
    #[error("root server error")]
    RootError(#[source] RootError),
    /// The client is unauthenticated
    #[error("unauthenticated client")]
    Unauthenticated,
    /// The server returned an unsuccessful HTTP status code. Some common codes include:
    ///
    /// * `429`: Too many requests in a short period of time
    /// * `500`: The server encountered an internal error
    /// * `502`: The server is currently unavailable or down
    #[error(
        "unsuccessful response: {}{}",
        .0.as_u16(),
        .0.canonical_reason().map_or(String::new(), |reason| " ".to_owned() + reason)
    )]
    UnsuccessfulResponse(StatusCode),
}
impl Error {
    pub(super) fn try_from_root<E: RequestError>(root_error: RootError) -> Result<Error>
    where
        for<'a> &'a E::Source: From<&'a RootError>,
    {
        Ok(match E::try_convert(<&E::Source>::from(&root_error))? {
            Some(error) => Error::RequestError(Box::new(error)),
            None => Error::RootError(root_error),
        })
    }

    pub(super) fn try_from_melior<E: RequestError>(melior_error: MeliorError) -> Result<Error>
    where
        for<'a> &'a E::Source: From<&'a MeliorError>,
    {
        Ok(match E::try_convert(<&E::Source>::from(&melior_error))? {
            Some(error) => Error::RequestError(Box::new(error)),
            None => Error::MeliorError(melior_error),
        })
    }
}

impl From<JwtError> for Error {
    fn from(value: JwtError) -> Self {
        Self::JwtError(Arc::new(value))
    }
}
