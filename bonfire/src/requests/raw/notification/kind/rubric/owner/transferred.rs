use serde::Deserialize;

use crate::models::notification::RubricOwnerTransferred;
use crate::requests::raw::{RawAccountRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawTransferred {
    #[serde(rename = "rubricId")]
    pub id: u64,
    #[serde(rename = "rubricName")]
    pub name: String,
    pub fandom_id: u64,
    #[serde(rename = "languageId")]
    pub fandom_language: RawLanguage,
    pub new_owner_id: u64,
    pub new_owner_name: String,
    pub moderation_id: u64,
    #[serde(rename = "adminId")]
    pub moderator_id: u64,
    #[serde(rename = "adminName")]
    pub moderator_name: String,
    #[serde(rename = "adminSex")]
    pub moderator_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawTransferred> for RubricOwnerTransferred {
    type Error = Error;

    fn try_from(value: RawTransferred) -> Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
            fandom_id: value.fandom_id,
            fandom_language: value.fandom_language.try_into()?,
            new_owner_id: value.new_owner_id,
            new_owner_name: value.new_owner_name,
            moderation_id: value.moderation_id,
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
