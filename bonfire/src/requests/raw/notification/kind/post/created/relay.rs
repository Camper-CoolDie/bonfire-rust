use serde::Deserialize;

use crate::models::notification::PostRelayPostCreated;
use crate::requests::raw::{RawFandomRef, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawRelay {
    pub post_id: u64,
    pub fandom_id: u64,
    #[serde(rename = "fandomLanguageId")]
    pub fandom_language: RawLanguage,
    pub fandom_name: String,
    #[serde(rename = "activityId")]
    pub relay_id: u64,
    #[serde(rename = "activityName")]
    pub relay_name: String,
}

impl TryFrom<RawRelay> for PostRelayPostCreated {
    type Error = Error;

    fn try_from(value: RawRelay) -> Result<Self> {
        Ok(Self {
            post_id: value.post_id,
            fandom: RawFandomRef {
                id: value.fandom_id,
                language: value.fandom_language,
                name: value.fandom_name,
            }
            .try_into()?,
            relay_id: value.relay_id,
            relay_name: value.relay_name,
        })
    }
}
