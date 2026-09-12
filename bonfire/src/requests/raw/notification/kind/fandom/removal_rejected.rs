use serde::Deserialize;

use crate::models::notification::FandomRemovalRejected;
use crate::requests::raw::{RawFandomRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawRemovalRejected {
    pub fandom_id: u64,
    #[serde(rename = "languageId")]
    pub fandom_language: RawLanguage,
    pub fandom_name: String,
    pub admin_name: String,
    #[serde(rename = "adminSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawRemovalRejected> for FandomRemovalRejected {
    type Error = Error;

    fn try_from(value: RawRemovalRejected) -> Result<Self> {
        Ok(Self {
            fandom: RawFandomRef {
                id: value.fandom_id,
                language: value.fandom_language,
                name: value.fandom_name,
            }
            .try_into()?,
            admin_name: value.admin_name,
            admin_gender: value.admin_gender.try_into()?,
            reason: value.reason,
        })
    }
}
