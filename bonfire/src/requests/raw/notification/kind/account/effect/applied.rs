use serde::Deserialize;

use crate::models::Effect;
use crate::requests::raw::RawEffect;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawEffectApplied {
    #[serde(rename = "mAccEffect")]
    pub effect: RawEffect,
}

impl TryFrom<RawEffectApplied> for Effect {
    type Error = Error;

    fn try_from(value: RawEffectApplied) -> Result<Self> {
        value.effect.try_into()
    }
}
