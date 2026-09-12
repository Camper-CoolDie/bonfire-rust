use serde::Deserialize;

use crate::models::notification::RubricFandomChanged;
use crate::requests::raw::{RawAccountRef, RawFandomRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawFandomChanged {
    #[serde(rename = "rubricId")]
    pub id: u64,
    #[serde(rename = "rubricName")]
    pub name: String,
    pub moderation_id: u64,
    pub admin_id: u64,
    pub admin_name: String,
    #[serde(rename = "adminSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "srcFandomId")]
    pub old_fandom_id: u64,
    #[serde(rename = "srcLanguageId")]
    pub old_fandom_language: RawLanguage,
    #[serde(rename = "srcFandomName")]
    pub old_fandom_name: String,
    #[serde(rename = "destFandomId")]
    pub new_fandom_id: u64,
    #[serde(rename = "destLanguageId")]
    pub new_fandom_language: RawLanguage,
    #[serde(rename = "destFandomName")]
    pub new_fandom_name: String,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawFandomChanged> for RubricFandomChanged {
    type Error = Error;

    fn try_from(value: RawFandomChanged) -> Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
            moderation_id: value.moderation_id,
            admin: RawAccountRef {
                id: value.admin_id,
                name: value.admin_name,
                gender: value.admin_gender,
            }
            .try_into()?,
            old_fandom: RawFandomRef {
                id: value.old_fandom_id,
                language: value.old_fandom_language,
                name: value.old_fandom_name,
            }
            .try_into()?,
            new_fandom: RawFandomRef {
                id: value.new_fandom_id,
                language: value.new_fandom_language,
                name: value.new_fandom_name,
            }
            .try_into()?,
            reason: value.reason,
        })
    }
}
