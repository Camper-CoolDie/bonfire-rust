use serde::Deserialize;

use crate::models::notification::PostNsfwToggled;
use crate::requests::raw::RawGender;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawNsfwToggled {
    #[serde(rename = "nsfw")]
    pub is_nsfw: bool,
    pub moderation_id: u64,
    pub moderator_name: String,
    #[serde(rename = "moderatorSex")]
    pub moderator_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawNsfwToggled> for PostNsfwToggled {
    type Error = Error;

    fn try_from(value: RawNsfwToggled) -> Result<Self> {
        Ok(Self {
            is_nsfw: value.is_nsfw,
            moderation_id: value.moderation_id,
            moderator_name: value.moderator_name,
            moderator_gender: value.moderator_gender.try_into()?,
            reason: value.reason,
        })
    }
}
