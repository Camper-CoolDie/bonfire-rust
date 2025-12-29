use std::result::Result as StdResult;

use http::uri::Scheme;
use thiserror::Error;

/// A type alias for [Result<T, connector::Error>][std::result::Result<T, Error>].
pub type Result<T> = StdResult<T, Error>;

/// Represents errors that can occur while connecting to the servers.
#[derive(Error, Debug)]
pub enum Error {
    /// URI host is not specified
    #[error("unspecified URI host")]
    EmptyHost,
    /// URI scheme is not specified
    #[error("unspecified URI scheme")]
    EmptyScheme,
    /// Can't handshake with the server
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    /// The provided host is not a valid DNS name
    #[error("invalid DNS name: {0}")]
    InvalidDnsName(String),
    /// Can't create a TCP connection
    #[error("IO error")]
    IoError(#[from] std::io::Error),
    /// Can't determine the settings for connecting
    #[error("TLS error")]
    TlsError(#[from] rustls::Error),
    /// URI scheme is not supported. It must be either `http` or `https`
    #[error("unsupported URI scheme {0:?}")]
    UnsupportedScheme(Scheme),
}
