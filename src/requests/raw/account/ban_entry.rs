use serde::Deserialize;

use crate::models::account::BanEntry;
use crate::requests::raw::RawAccount;
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawBanEntry {
    pub account: RawAccount,
    #[serde(rename = "banDate")]
    pub banned_until: i64,
}

impl TryFrom<RawBanEntry> for BanEntry {
    type Error = Error;

    fn try_from(value: RawBanEntry) -> Result<Self> {
        Ok(Self {
            account: value.account.try_into()?,
            banned_until: timestamp_from_millis(value.banned_until)?,
        })
    }
}
