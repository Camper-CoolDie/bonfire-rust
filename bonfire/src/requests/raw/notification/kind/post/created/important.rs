use serde::Deserialize;

use crate::models::notification::ImportantPostCreated;
use crate::requests::raw::{RawFandomRef, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawImportantPostCreated {
    #[serde(rename = "unitId")]
    pub id: u64,
    pub fandom_id: u64,
    #[serde(rename = "fandomLanguageId")]
    pub fandom_language: RawLanguage,
    pub fandom_name: String,
    #[serde(rename = "moderatorAccountId")]
    pub importance_moderator_id: u64,
    #[serde(rename = "comment")]
    pub importance_reason: String,
}

impl TryFrom<RawImportantPostCreated> for ImportantPostCreated {
    type Error = Error;

    fn try_from(value: RawImportantPostCreated) -> Result<Self> {
        Ok(Self {
            id: value.id,
            fandom: RawFandomRef {
                id: value.fandom_id,
                language: value.fandom_language,
                name: value.fandom_name,
            }
            .try_into()?,
            importance_moderator_id: value.importance_moderator_id,
            importance_reason: value.importance_reason,
        })
    }
}
