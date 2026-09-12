use serde::Deserialize;

use crate::models::notification::RubricKarmaCoefChanged;
use crate::requests::raw::RawLanguage;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawKarmaCoefChanged {
    #[serde(rename = "rubricId")]
    pub id: u64,
    #[serde(rename = "rubricName")]
    pub name: String,
    pub fandom_id: u64,
    #[serde(rename = "languageId")]
    pub fandom_language: RawLanguage,
    #[serde(rename = "newCof")]
    pub new_coef: f64,
    #[serde(rename = "cofChange")]
    pub coef_change: f64,
}

impl TryFrom<RawKarmaCoefChanged> for RubricKarmaCoefChanged {
    type Error = Error;

    fn try_from(value: RawKarmaCoefChanged) -> Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
            fandom_id: value.fandom_id,
            fandom_language: value.fandom_language.try_into()?,
            old_coef: (value.new_coef - value.coef_change) / 100.0,
            new_coef: value.new_coef / 100.0,
        })
    }
}
