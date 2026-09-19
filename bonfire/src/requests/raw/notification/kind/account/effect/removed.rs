use serde::Deserialize;

use crate::models::notification::EffectRemoved;
use crate::requests::raw::RawGender;
use crate::requests::raw::account::RawEffectKind;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawEffectRemoved {
    #[serde(rename = "effectId")]
    pub id: u64,
    #[serde(rename = "effectIndex")]
    pub kind: RawEffectKind,
    pub admin_name: String,
    #[serde(rename = "adminSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawEffectRemoved> for EffectRemoved {
    type Error = Error;

    fn try_from(value: RawEffectRemoved) -> Result<Self> {
        Ok(Self {
            id: value.id,
            kind: value.kind.into(),
            admin_name: value.admin_name,
            admin_gender: value.admin_gender.try_into()?,
            reason: value.reason,
        })
    }
}
