use std::result::Result as StdResult;

use http::StatusCode;
use thiserror::Error;

use crate::connector;
use crate::models::{auth, MeliorError, RootError};

/// A type alias for [Result<T, Error>][std::result::Result<T, Error>].
pub type Result<T> = StdResult<T, Error>;

/// Represents errors that can occur while operating with a client.
///
/// # Source
///
/// An `Error` can be the result of operations like connecting or sending a request.
#[derive(Error, Debug)]
pub enum Error {
    /// Can't authenticate (no session?)
    #[error("authentication error")]
    AuthError(#[from] auth::Error),
    /// Can't connect to the servers
    #[error("connection error")]
    ConnectorError(#[from] connector::Error),
    /// Can't (de)serialize a JSON object
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
    /// Can't decode a JSON Web Token
    #[error("JWT error")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    /// Can't build a request (check the method arguments)
    #[error("HTTP error")]
    HttpError(#[from] http::Error),
    /// Can't send or receive data from the server
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    /// Can't write data into a buffer (or some other IO-related error)
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    /// The melior server returned an error
    #[error("melior server error")]
    MeliorError(#[from] MeliorError),
    /// The root server returned an error
    #[error("root server error")]
    RootError(#[from] RootError),
    /// The server returned an unsuccessful HTTP status code. Some of the most common codes are:
    ///
    /// * `429`: Too many requests in a short period of time
    /// * `500`: The server couldn't handle the request
    /// * `502`: The server is down
    #[error("unsuccessful response: {0:?}")]
    UnsuccessfulResponse(StatusCode),
}
