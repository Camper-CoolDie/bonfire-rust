use serde::Deserialize;

use crate::models::notification::AccountFandomUnbanned;
use crate::requests::raw::{RawAccountRef, RawFandomRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawFandomUnbanned {
    pub fandom_id: u64,
    #[serde(rename = "languageId")]
    pub fandom_language: RawLanguage,
    pub fandom_name: String,
    pub moderator_id: u64,
    pub moderator_name: String,
    #[serde(rename = "moderatorSex")]
    pub moderator_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawFandomUnbanned> for AccountFandomUnbanned {
    type Error = Error;

    fn try_from(value: RawFandomUnbanned) -> Result<Self> {
        Ok(Self {
            fandom: RawFandomRef {
                id: value.fandom_id,
                language: value.fandom_language,
                name: value.fandom_name,
            }
            .try_into()?,
            moderator: RawAccountRef {
                id: value.moderator_id,
                name: value.moderator_name,
                gender: value.moderator_gender,
            }
            .try_into()?,
            reason: value.reason,
        })
    }
}
