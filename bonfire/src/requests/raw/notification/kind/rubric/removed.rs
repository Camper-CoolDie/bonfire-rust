use serde::Deserialize;

use crate::models::notification::RubricRemoved;
use crate::requests::raw::{RawAccountRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawRemoved {
    #[serde(rename = "rubricId")]
    pub id: u64,
    #[serde(rename = "rubricName")]
    pub name: String,
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

impl TryFrom<RawRemoved> for RubricRemoved {
    type Error = Error;

    fn try_from(value: RawRemoved) -> Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
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
