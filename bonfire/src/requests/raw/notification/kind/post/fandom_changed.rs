use serde::Deserialize;

use crate::models::notification::PostFandomChanged;
use crate::requests::raw::{RawAccountRef, RawFandomRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawFandomChanged {
    #[serde(rename = "unitId")]
    pub post_id: u64,
    pub old_fandom_id: u64,
    #[serde(rename = "oldLanguageId")]
    pub old_fandom_language: RawLanguage,
    pub old_fandom_name: String,
    pub new_fandom_id: u64,
    #[serde(rename = "newLanguageId")]
    pub new_fandom_language: RawLanguage,
    pub new_fandom_name: String,
    pub admin_id: u64,
    pub admin_name: String,
    #[serde(rename = "adminSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawFandomChanged> for PostFandomChanged {
    type Error = Error;

    fn try_from(value: RawFandomChanged) -> Result<Self> {
        Ok(Self {
            post_id: value.post_id,
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
            admin: RawAccountRef {
                id: value.admin_id,
                name: value.admin_name,
                gender: value.admin_gender,
            }
            .try_into()?,
            reason: value.reason,
        })
    }
}
