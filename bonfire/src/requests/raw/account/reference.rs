use serde::Deserialize;

use crate::models::AccountRef;
use crate::requests::raw::RawGender;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawReference {
    #[serde(rename = "J_ACCOUNT_ID")]
    pub id: u64,
    #[serde(rename = "J_ACCOUNT_NAME")]
    pub name: String,
    #[serde(rename = "accountSex")]
    pub gender: RawGender,
}

impl TryFrom<RawReference> for AccountRef {
    type Error = Error;

    fn try_from(value: RawReference) -> Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
            gender: value.gender.try_into()?,
        })
    }
}

impl TryFrom<RawReference> for Option<AccountRef> {
    type Error = Error;

    fn try_from(value: RawReference) -> Result<Self> {
        Ok(match value.id {
            0 => None,
            _ => Some(value.try_into()?),
        })
    }
}
