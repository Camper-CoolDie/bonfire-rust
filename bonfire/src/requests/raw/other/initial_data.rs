use serde::Deserialize;

use crate::models::InitialData;
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::requests::raw::{RawAccount, RawSettings};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawInitialData {
    pub account: RawAccount,
    pub settings: RawSettings,
    #[serde(rename = "protoadmins")]
    pub protoadmin_ids: Vec<u64>,
    pub server_time: i64,
    #[serde(rename = "hasSubscribes")]
    pub has_follows: bool,
}

impl TryFrom<RawInitialData> for InitialData {
    type Error = Error;

    fn try_from(value: RawInitialData) -> Result<Self> {
        Ok(Self {
            account: value.account.try_into()?,
            settings: value.settings.try_into()?,
            protoadmin_ids: value.protoadmin_ids,
            server_time: timestamp_from_millis(value.server_time)?,
            has_follows: value.has_follows,
        })
    }
}
