use serde::Deserialize;

use crate::models::notification::PostVisibilityChanged;
use crate::requests::raw::RawGender;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawVisibilityChanged {
    pub moderation_id: u64,
    pub moderator_name: String,
    #[serde(rename = "moderatorSex")]
    pub moderator_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawVisibilityChanged> for PostVisibilityChanged {
    type Error = Error;

    fn try_from(value: RawVisibilityChanged) -> Result<Self> {
        Ok(Self {
            moderation_id: value.moderation_id,
            moderator_name: value.moderator_name,
            moderator_gender: value.moderator_gender.try_into()?,
            reason: value.reason,
        })
    }
}
