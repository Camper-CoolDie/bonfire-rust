use serde::Deserialize;

use crate::models::notification::FandomModeratorSet;
use crate::requests::raw::{RawFandomRef, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawModeratorSet {
    pub fandom_id: u64,
    #[serde(rename = "languageId")]
    pub fandom_language: RawLanguage,
    pub fandom_name: String,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawModeratorSet> for FandomModeratorSet {
    type Error = Error;

    fn try_from(value: RawModeratorSet) -> Result<Self> {
        Ok(Self {
            fandom: RawFandomRef {
                id: value.fandom_id,
                language: value.fandom_language,
                name: value.fandom_name,
            }
            .try_into()?,
            reason: value.reason,
        })
    }
}
