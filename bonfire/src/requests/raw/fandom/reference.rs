use serde::Deserialize;

use crate::models::FandomRef;
use crate::requests::raw::RawLanguage;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawFandomRef {
    pub id: u64,
    pub language: RawLanguage,
    pub name: String,
}

impl TryFrom<RawFandomRef> for FandomRef {
    type Error = Error;

    fn try_from(value: RawFandomRef) -> Result<Self> {
        Ok(Self {
            id: value.id,
            language: value.language.try_into()?,
            name: value.name,
        })
    }
}
