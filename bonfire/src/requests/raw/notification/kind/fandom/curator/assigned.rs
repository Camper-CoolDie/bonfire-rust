use serde::Deserialize;

use crate::models::notification::FandomCuratorAssigned;
use crate::requests::raw::{RawAccountRef, RawFandomRef, RawGender, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawAssigned {
    #[serde(rename = "oldAccountId")]
    pub old_curator_id: u64,
    pub fandom_id: u64,
    #[serde(rename = "languageId")]
    pub fandom_language: RawLanguage,
    pub fandom_name: String,
    #[serde(rename = "adminAcccountId")]
    pub admin_id: u64,
    #[serde(rename = "adminAcccountName")]
    pub admin_name: String,
    #[serde(rename = "adminAcccountSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawAssigned> for FandomCuratorAssigned {
    type Error = Error;

    fn try_from(value: RawAssigned) -> Result<Self> {
        Ok(Self {
            old_curator_id: match value.old_curator_id {
                0 => None,
                id => Some(id),
            },
            fandom: RawFandomRef {
                id: value.fandom_id,
                language: value.fandom_language,
                name: value.fandom_name,
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
