use serde::Deserialize;

use crate::models::AccountRef;
use crate::requests::raw::RawGender;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawAccountRef {
    pub id: u64,
    pub name: String,
    pub gender: RawGender,
}

impl TryFrom<RawAccountRef> for AccountRef {
    type Error = Error;

    fn try_from(value: RawAccountRef) -> Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
            gender: value.gender.try_into()?,
        })
    }
}
