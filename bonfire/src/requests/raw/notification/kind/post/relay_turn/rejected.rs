use serde::Deserialize;

use crate::models::notification::PostRelayTurnRejected;
use crate::requests::raw::{RawAccountRef, RawFandomRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawRelayTurnRejected {
    #[serde(rename = "rejectedAccountId")]
    pub account_id: u64,
    #[serde(rename = "rejectedAccountName")]
    pub account_name: String,
    #[serde(rename = "rejectedAccountSex")]
    pub account_gender: RawGender,
    #[serde(rename = "activityId")]
    pub id: u64,
    #[serde(rename = "activityName")]
    pub name: String,
    pub fandom_id: u64,
    #[serde(rename = "fandomLanguageId")]
    pub fandom_language: RawLanguage,
    pub fandom_name: String,
    #[serde(rename = "newAccountId")]
    pub next_account_id: u64,
    #[serde(rename = "newAccountName")]
    pub next_account_name: String,
    #[serde(rename = "newAccountSex")]
    pub next_account_gender: RawGender,
}

impl TryFrom<RawRelayTurnRejected> for PostRelayTurnRejected {
    type Error = Error;

    fn try_from(value: RawRelayTurnRejected) -> Result<Self> {
        Ok(Self {
            account: RawAccountRef {
                id: value.account_id,
                name: value.account_name,
                gender: value.account_gender,
            }
            .try_into()?,
            id: value.id,
            name: value.name,
            fandom: RawFandomRef {
                id: value.fandom_id,
                language: value.fandom_language,
                name: value.fandom_name,
            }
            .try_into()?,
            next_account: RawAccountRef {
                id: value.next_account_id,
                name: value.next_account_name,
                gender: value.next_account_gender,
            }
            .try_into()?,
        })
    }
}
