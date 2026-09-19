use serde::Deserialize;

use crate::models::notification::PostRelayTurnAssigned;
use crate::requests::raw::{RawAccountRef, RawFandomRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawRelayTurnAssigned {
    #[serde(rename = "fromAccountId")]
    pub account_id: u64,
    #[serde(rename = "fromAccountName")]
    pub account_name: String,
    #[serde(rename = "fromAccountSex")]
    pub account_gender: RawGender,
    #[serde(rename = "activityId")]
    pub id: u64,
    #[serde(rename = "activityName")]
    pub name: String,
    pub fandom_id: u64,
    #[serde(rename = "fandomLanguageId")]
    pub fandom_language: RawLanguage,
    pub fandom_name: String,
}

impl TryFrom<RawRelayTurnAssigned> for PostRelayTurnAssigned {
    type Error = Error;

    fn try_from(value: RawRelayTurnAssigned) -> Result<Self> {
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
        })
    }
}
