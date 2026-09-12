use serde::Deserialize;

use crate::models::notification::PostImagesPurged;
use crate::requests::raw::RawGender;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawImagesPurged {
    pub post_id: u64,
    pub admin_name: String,
    #[serde(rename = "adminSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawImagesPurged> for PostImagesPurged {
    type Error = Error;

    fn try_from(value: RawImagesPurged) -> Result<Self> {
        Ok(Self {
            post_id: value.post_id,
            admin_name: value.admin_name,
            admin_gender: value.admin_gender.try_into()?,
            reason: value.reason,
        })
    }
}
