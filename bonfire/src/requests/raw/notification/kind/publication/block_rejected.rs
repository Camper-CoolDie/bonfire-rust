use serde::Deserialize;

use crate::models::notification::PublicationBlockRejected;
use crate::requests::raw::{RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawBlockRejected {
    pub moderation_id: u64,
    pub fandom_id: u64,
    #[serde(rename = "languageId")]
    pub fandom_language: RawLanguage,
    pub admin_name: String,
    #[serde(rename = "adminSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawBlockRejected> for PublicationBlockRejected {
    type Error = Error;

    fn try_from(value: RawBlockRejected) -> Result<Self> {
        Ok(Self {
            moderation_id: value.moderation_id,
            fandom_id: value.fandom_id,
            fandom_language: value.fandom_language.try_into()?,
            admin_name: value.admin_name,
            admin_gender: value.admin_gender.try_into()?,
            reason: value.reason,
        })
    }
}
