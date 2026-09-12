use serde::Deserialize;

use crate::models::notification::ProfileFieldSet;
use crate::requests::raw::RawGender;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawFieldSet {
    pub admin_name: String,
    #[serde(rename = "adminSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawFieldSet> for ProfileFieldSet {
    type Error = Error;

    fn try_from(value: RawFieldSet) -> Result<Self> {
        Ok(Self {
            admin_name: value.admin_name,
            admin_gender: value.admin_gender.try_into()?,
            reason: value.reason,
        })
    }
}
