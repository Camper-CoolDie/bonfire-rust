use std::result::Result as StdResult;

use chrono::{DateTime, Utc};
use jsonwebtoken::errors::{ErrorKind, Result as JwtResult};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::Error;
use serde::{Deserialize, Deserializer};

use crate::models::Auth;

const JWT_ISSUER: &str = "https://bonfire.moe";
const JWT_ACCESS_AUDIENCE: &str = "access";

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

fn decode(token: &str) -> JwtResult<TokenClaims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[JWT_ACCESS_AUDIENCE]);
    validation.set_issuer(&[JWT_ISSUER]);
    validation.insecure_disable_signature_validation();
    Ok(jsonwebtoken::decode(token, &DecodingKey::from_secret(&[]), &validation)?.claims)
}

pub(super) fn validate_token(auth: &Auth, just_refreshed: bool) -> JwtResult<Option<String>> {
    // Ok(Some(...)) => token is valid
    // Ok(None) => needs refreshing (only if just_refreshed is false)
    // Err(...) => error while validating
    decode(&auth.access_token)
        .map(Some)
        .or_else(|error| match error.kind() {
            ErrorKind::ExpiredSignature => {
                (!just_refreshed).then_some(Ok(None)).ok_or(error).flatten()
            }
            _ => Err(error),
        })
        .map(|option| {
            option.map(|claims| {
                tracing::debug!(
                    subject = claims.subject,
                    expires_at = ?claims.expires_at,
                    issued_at = ?claims.issued_at,
                    "validated login"
                );
                auth.access_token.clone()
            })
        })
        .inspect_err(|error| {
            tracing::error!(
                ?error,
                "failed to validate login{}",
                if just_refreshed {
                    " after refreshing"
                } else {
                    ""
                }
            );
        })
}
