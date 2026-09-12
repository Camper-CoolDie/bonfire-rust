use serde::Deserialize;

use crate::models::notification::FollowedPostCreated;
use crate::requests::raw::{RawAccountRef, RawGender};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawFollowed {
    #[serde(rename = "J_UNIT_ID")]
    pub id: u64,
    #[serde(rename = "J_ACCOUNT_ID")]
    pub author_id: u64,
    #[serde(rename = "J_ACCOUNT_NAME")]
    pub author_name: String,
    #[serde(rename = "accountSex")]
    pub author_gender: RawGender,
}

impl TryFrom<RawFollowed> for FollowedPostCreated {
    type Error = Error;

    fn try_from(value: RawFollowed) -> Result<Self> {
        Ok(Self {
            id: value.id,
            author: RawAccountRef {
                id: value.author_id,
                name: value.author_name,
                gender: value.author_gender,
            }
            .try_into()?,
        })
    }
}
