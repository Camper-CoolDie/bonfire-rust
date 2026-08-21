use std::result::Result as StdResult;

use thiserror::Error;

/// A type alias for [`Result<T, Error>`][StdResult].
pub type Result<T> = StdResult<T, Error>;

/// Represents an error that can occur while parsing or validating JSON Web Tokens (JWTs).
#[derive(Error, Debug)]
pub enum Error {
    /// The provided access token does not contain a payload
    #[error("no payload")]
    NoPayload,
    /// An error occurred during Base64 decoding of the token payload
    #[error("base64 error")]
    Base64Error(#[from] base64::DecodeError),
    /// An error occurred during JSON deserialization of the token claims
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
}
