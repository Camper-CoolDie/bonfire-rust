use serde::Deserialize;

use crate::models::notification::PostTagsChanged;
use crate::requests::raw::{RawAccountRef, RawGender};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawTagsChanged {
    pub moderation_id: u64,
    pub moderator_id: u64,
    pub moderator_name: String,
    #[serde(rename = "moderatorSex")]
    pub moderator_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawTagsChanged> for PostTagsChanged {
    type Error = Error;

    fn try_from(value: RawTagsChanged) -> Result<Self> {
        Ok(Self {
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
