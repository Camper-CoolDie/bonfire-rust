use std::result::Result as StdResult;

use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use chrono::{DateTime, Utc};
use serde::de::Error;
use serde::{Deserialize, Deserializer};
use thiserror::Error;

fn deserialize_timestamp<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> StdResult<DateTime<Utc>, D::Error> {
    let seconds = i64::deserialize(deserializer)?;
    DateTime::from_timestamp(seconds, 0)
        .ok_or_else(|| D::Error::custom(format!("timestamp {} is out of range", seconds)))
}

pub(super) type JwtResult<T> = StdResult<T, JwtError>;

/// Represents an error parsing the authentication credentials (more specifically, parsing its
/// access token).
#[derive(Error, Debug)]
pub enum JwtError {
    /// The contained access token doesn't have a payload
    #[error("no payload")]
    NoPayload,
    /// Can't decode payload of the contained access token
    #[error("base64 error")]
    Base64Error(#[from] base64::DecodeError),
    /// Can't deserialize the contained access token
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Deserialize)]
pub(super) struct JwtClaims {
    #[serde(rename = "sub")]
    pub subject: String,
    #[serde(rename = "exp", deserialize_with = "deserialize_timestamp")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "iat", deserialize_with = "deserialize_timestamp")]
    pub issued_at: DateTime<Utc>,
    // There are other fields, but we don't need them yet
}

pub(super) fn decode_token(token: &str) -> JwtResult<JwtClaims> {
    token
        .split('.')
        .nth(1)
        .ok_or(JwtError::NoPayload)
        .and_then(|data| BASE64_URL_SAFE_NO_PAD.decode(data).map_err(JwtError::from))
        .and_then(|decoded| serde_json::from_slice::<JwtClaims>(&decoded).map_err(JwtError::from))
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
