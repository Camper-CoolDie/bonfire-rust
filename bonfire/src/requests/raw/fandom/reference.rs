use serde::Deserialize;

use crate::models::FandomRef;
use crate::requests::raw::RawLanguage;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawReference {
    #[serde(rename = "fandomId")]
    pub id: u64,
    #[serde(rename = "languageId")]
    pub language: RawLanguage,
    #[serde(rename = "fandomName")]
    pub name: String,
}

impl TryFrom<RawReference> for FandomRef {
    type Error = Error;

    fn try_from(value: RawReference) -> Result<Self> {
        Ok(Self {
            id: value.id,
            language: value.language.try_into()?,
            name: value.name,
        })
    }
}

impl TryFrom<RawReference> for Option<FandomRef> {
    type Error = Error;

    fn try_from(value: RawReference) -> Result<Self> {
        Ok(match value.id {
            0 => None,
            _ => Some(value.try_into()?),
        })
    }
}
