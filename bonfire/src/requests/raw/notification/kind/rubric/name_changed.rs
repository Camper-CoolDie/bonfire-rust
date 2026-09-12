use serde::Deserialize;

use crate::models::notification::RubricNameChanged;
use crate::requests::raw::{RawAccountRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawNameChanged {
    #[serde(rename = "rubricId")]
    pub id: u64,
    #[serde(rename = "rubricOldName")]
    pub old_name: String,
    #[serde(rename = "rubricNewName")]
    pub new_name: String,
    pub fandom_id: u64,
    #[serde(rename = "languageId")]
    pub fandom_language: RawLanguage,
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

impl TryFrom<RawNameChanged> for RubricNameChanged {
    type Error = Error;

    fn try_from(value: RawNameChanged) -> Result<Self> {
        Ok(Self {
            id: value.id,
            old_name: value.old_name,
            new_name: value.new_name,
            fandom_id: value.fandom_id,
            fandom_language: value.fandom_language.try_into()?,
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
