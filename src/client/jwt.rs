use std::result::Result as StdResult;

use chrono::{DateTime, Utc};
use jsonwebtoken::errors::Result as JwtResult;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use serde::de::Error;
use serde::{Deserialize, Deserializer};

const JWT_ISSUER: &str = "https://bonfire.moe";
const JWT_ACCESS_AUDIENCE: &str = "access";

#[derive(Deserialize)]
pub(super) struct TokenClaims {
    #[serde(rename = "sub")]
    pub subject: String,
    #[serde(
        rename = "exp",
        deserialize_with = "TokenClaims::deserialize_timestamp"
    )]
    pub expires_at: DateTime<Utc>,
    #[serde(
        rename = "iat",
        deserialize_with = "TokenClaims::deserialize_timestamp"
    )]
    pub issued_at: DateTime<Utc>,
    // There are other fields, but we don't need them yet
}
impl TokenClaims {
    fn deserialize_timestamp<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> StdResult<DateTime<Utc>, D::Error> {
        DateTime::from_timestamp(i64::deserialize(deserializer)?, 0)
            .ok_or_else(|| D::Error::custom("timestamp is out of range"))
    }
}

pub(super) fn decode(token: &str) -> JwtResult<TokenClaims> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[JWT_ACCESS_AUDIENCE]);
    validation.set_issuer(&[JWT_ISSUER]);
    validation.insecure_disable_signature_validation();
    Ok(jsonwebtoken::decode(token, &DecodingKey::from_secret(&[]), &validation)?.claims)
}
