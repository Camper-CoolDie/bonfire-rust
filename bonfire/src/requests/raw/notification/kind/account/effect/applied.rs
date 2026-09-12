use serde::Deserialize;

use crate::models::Effect;
use crate::requests::raw::RawEffect;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawApplied {
    #[serde(rename = "mAccEffect")]
    pub effect: RawEffect,
}

impl TryFrom<RawApplied> for Effect {
    type Error = Error;

    fn try_from(value: RawApplied) -> Result<Self> {
        value.effect.try_into()
    }
}
