use std::result::Result as StdResult;

use chrono::{DateTime, Utc};
use serde::de::Error as _;
use serde::{Deserialize, Deserializer};

fn deserialize_timestamp<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> StdResult<DateTime<Utc>, D::Error> {
    let seconds = i64::deserialize(deserializer)?;
    DateTime::from_timestamp(seconds, 0)
        .ok_or_else(|| D::Error::custom(format!("timestamp {seconds} is out of range")))
}

#[derive(Debug, Deserialize)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub subject: Option<String>,
    #[serde(rename = "exp", deserialize_with = "deserialize_timestamp")]
    pub expires_at: DateTime<Utc>,
    // There are other fields, but we don't need them yet
}
