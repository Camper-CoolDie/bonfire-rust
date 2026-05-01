use std::result::Result as StdResult;

use base64::Engine as _;
use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use chrono::{DateTime, Utc};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

fn deserialize_timestamp<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> StdResult<DateTime<Utc>, D::Error> {
    let seconds = i64::deserialize(deserializer)?;
    DateTime::from_timestamp(seconds, 0)
        .ok_or_else(|| D::Error::custom(format!("timestamp {seconds} is out of range")))
}

pub(super) type Result<T> = StdResult<T, Error>;

/// Represents an error that can occur while parsing or validating JSON Web Tokens (JWT).
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

#[derive(Debug, Deserialize)]
pub(super) struct Claims {
    #[serde(rename = "sub")]
    pub subject: String,
    #[serde(rename = "exp", deserialize_with = "deserialize_timestamp")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "iat", deserialize_with = "deserialize_timestamp")]
    pub issued_at: DateTime<Utc>,
    // There are other fields, but we don't need them yet
}

pub(super) fn decode_token(token: &str) -> Result<Claims> {
    token
        .split('.')
        .nth(1)
        .ok_or(Error::NoPayload)
        .and_then(|data| BASE64_URL_SAFE_NO_PAD.decode(data).map_err(Error::from))
        .and_then(|decoded| serde_json::from_slice::<Claims>(&decoded).map_err(Error::from))
        .inspect(|claims| {
            tracing::debug!(
                subject = claims.subject,
                expires_at = ?claims.expires_at,
                issued_at = ?claims.issued_at,
                "decoded token"
            );
        })
        .inspect_err(|error| tracing::error!(?error, "failed to decode token"))
}
