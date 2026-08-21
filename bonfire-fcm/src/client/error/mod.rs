mod android_registration;

use std::error::Error as StdError;
use std::io;
use std::result::Result as StdResult;
use std::sync::Arc;

pub use android_registration::AndroidRegistrationError;
use http::StatusCode;
use thiserror::Error;

pub type Result<T> = StdResult<T, Error>;

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum Error {
    #[error("android registration error")]
    AndroidRegistrationError(#[from] AndroidRegistrationError),
    #[error("Base64 decode error")]
    Base64Error(#[from] base64::DecodeError),
    #[error("ECE error")]
    EceError(#[from] ece::Error),
    #[error("HTTP error")]
    HttpError(#[from] http::Error),
    #[error("hyper client error")]
    HyperClientError(#[from] hyper_util::client::legacy::Error),
    #[error("hyper error")]
    HyperError(#[from] hyper::Error),
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
    #[error("JWT error")]
    JwtError(#[from] Arc<jwt::Error>),
    #[error(
        "MCS login error: {}{}",
        .kind.as_ref().unwrap_or(&.code.to_string()),
        .message.as_ref().map(|message| format!(" ({message})")).unwrap_or_default()
    )]
    McsLoginError {
        code: i32,
        message: Option<String>,
        kind: Option<String>,
    },
    #[error("MCS protocol error ({0})")]
    McsProtocolError(String),
    #[error("Protobuf decode error")]
    ProtobufDecodeError(#[from] prost::DecodeError),
    #[error("Protobuf encode error")]
    ProtobufEncodeError(#[from] prost::EncodeError),
    #[error(
        "unsuccessful response: {}{}",
        .0.as_u16(),
        .0.canonical_reason().map_or(String::new(), |reason| " ".to_owned() + reason)
    )]
    UnsuccessfulResponse(StatusCode),
    #[error("URL-encoded deserialize error")]
    UrlEncodedDeserializeError(#[from] serde_urlencoded::de::Error),
    #[error("URL-encoded serialize error")]
    UrlEncodedSerializeError(#[from] serde_urlencoded::ser::Error),
}
impl Error {
    pub(crate) fn duplicate_field(key: &'static str, source: &'static str) -> Self {
        Self::McsProtocolError(format!("duplicate field `{key}` while parsing {source}"))
    }

    pub(crate) fn missing_field(key: &'static str, source: &'static str) -> Self {
        Self::McsProtocolError(format!("missing field `{key}` while parsing {source}"))
    }

    pub(crate) fn invalid_format(key: &'static str, source: &'static str) -> Self {
        Self::McsProtocolError(format!(
            "invalid `{key}` value format while parsing {source}"
        ))
    }

    pub(crate) fn conversion<E: StdError>(
        error: E,
        key: &'static str,
        target: &'static str,
        source: &'static str,
    ) -> Self {
        Self::McsProtocolError(format!(
            "failed to convert field `{key}` to {target} while parsing {source}: {error}"
        ))
    }
}

impl From<jwt::Error> for Error {
    fn from(value: jwt::Error) -> Self {
        Self::JwtError(Arc::new(value))
    }
}
