use std::result::Result as StdResult;

use chrono::{DateTime, Utc};
use jsonwebtoken::errors::{ErrorKind, Result as JwtResult};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use once_cell::sync::Lazy;
use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::models::Auth;

const JWT_ISSUER: &str = "https://bonfire.moe";
const JWT_ACCESS_AUDIENCE: &str = "access";
static VALIDATION: Lazy<Validation> = Lazy::new(|| {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[JWT_ACCESS_AUDIENCE]);
    validation.set_issuer(&[JWT_ISSUER]);
    validation.insecure_disable_signature_validation();
    validation
});

fn deserialize_timestamp<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> StdResult<DateTime<Utc>, D::Error> {
    let millis = i64::deserialize(deserializer)?;
    DateTime::from_timestamp(millis, 0)
        .ok_or_else(|| D::Error::custom(format!("timestamp {} is out of range", millis)))
}

#[derive(Deserialize)]
struct TokenClaims {
    #[serde(rename = "sub")]
    pub subject: String,
    #[serde(rename = "exp", deserialize_with = "deserialize_timestamp")]
    pub expires_at: DateTime<Utc>,
    #[serde(rename = "iat", deserialize_with = "deserialize_timestamp")]
    pub issued_at: DateTime<Utc>,
    // There are other fields, but we don't need them yet
}

pub(super) fn is_token_expired(auth: &Auth) -> JwtResult<bool> {
    match validate_token(auth) {
        Ok(_) => Ok(false),
        Err(error) if error.kind() == &ErrorKind::ExpiredSignature => Ok(true),
        Err(error) => Err(error),
    }
}

fn validate_token(auth: &Auth) -> JwtResult<TokenClaims> {
    Ok(jsonwebtoken::decode::<TokenClaims>(
        &auth.access_token,
        &DecodingKey::from_secret(&[]),
        &VALIDATION,
    )?
    .claims)
    .inspect(|claims| {
        tracing::debug!(
            subject = claims.subject,
            expires_at = ?claims.expires_at,
            issued_at = ?claims.issued_at,
            "Validated login"
        );
    })
    .inspect_err(|error| tracing::error!(?error, "Failed to validate login"))
}
