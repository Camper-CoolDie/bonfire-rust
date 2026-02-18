use chrono::DateTime;
use serde::de::Error as _;
use serde::Deserialize;

use crate::models::account::PrisonEntry;
use crate::requests::raw::RawAccount;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawPrisonEntry {
    account: RawAccount,
    #[serde(rename = "banDate")]
    banned_until: i64,
}

impl TryFrom<RawPrisonEntry> for PrisonEntry {
    type Error = Error;

    fn try_from(value: RawPrisonEntry) -> Result<Self> {
        Ok(Self {
            account: value.account.try_into()?,
            banned_until: DateTime::from_timestamp_millis(value.banned_until).ok_or_else(|| {
                serde_json::Error::custom(format!(
                    "timestamp {} is out of range",
                    value.banned_until
                ))
            })?,
        })
    }
}
