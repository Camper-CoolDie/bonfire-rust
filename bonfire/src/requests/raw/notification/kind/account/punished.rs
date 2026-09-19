use serde::Deserialize;

use crate::models::notification::AccountPunished;
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawAccountPunished {
    #[serde(rename = "J_BLOCK_ACCOUNT_DATE")]
    pub banned_until: i64,
    #[serde(rename = "J_COMMENT")]
    pub reason: String,
}

impl TryFrom<RawAccountPunished> for AccountPunished {
    type Error = Error;

    fn try_from(value: RawAccountPunished) -> Result<Self> {
        Ok(Self {
            banned_until: match value.banned_until {
                -1 | 0 => None,
                millis => Some(timestamp_from_millis(millis)?),
            },
            reason: value.reason,
        })
    }
}
