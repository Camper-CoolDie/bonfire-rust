use serde::Deserialize;

use crate::models::notification::PostMultilingualDisabled;
use crate::requests::raw::RawGender;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawMultilingualDisabled {
    pub moderation_id: u64,
    pub moderator_name: String,
    #[serde(rename = "moderatorSex")]
    pub moderator_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawMultilingualDisabled> for PostMultilingualDisabled {
    type Error = Error;

    fn try_from(value: RawMultilingualDisabled) -> Result<Self> {
        Ok(Self {
            moderation_id: value.moderation_id,
            moderator_name: value.moderator_name,
            moderator_gender: value.moderator_gender.try_into()?,
            reason: value.reason,
        })
    }
}
