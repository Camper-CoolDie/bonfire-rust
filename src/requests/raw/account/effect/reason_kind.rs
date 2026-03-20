use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::account::EffectReasonKind;
use crate::{Error, Result};

pub(crate) enum RawEffectReasonKind {
    Gods,
    RejectedBlocks,
    TooManyBlocks,
    Swearing,
    Hater,
    Uncultured,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawEffectReasonKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawEffectReasonKind::Gods,
            2 => RawEffectReasonKind::RejectedBlocks,
            3 => RawEffectReasonKind::TooManyBlocks,
            4 => RawEffectReasonKind::Swearing,
            5 => RawEffectReasonKind::Hater,
            6 => RawEffectReasonKind::Uncultured,
            other => RawEffectReasonKind::Unknown(other),
        })
    }
}

impl TryFrom<RawEffectReasonKind> for Option<EffectReasonKind> {
    type Error = Error;

    fn try_from(value: RawEffectReasonKind) -> Result<Self> {
        Ok(match value {
            RawEffectReasonKind::Unknown(0) => None,
            RawEffectReasonKind::Gods => Some(EffectReasonKind::Gods),
            RawEffectReasonKind::RejectedBlocks => Some(EffectReasonKind::RejectedBlocks),
            RawEffectReasonKind::TooManyBlocks => Some(EffectReasonKind::TooManyBlocks),
            RawEffectReasonKind::Swearing => Some(EffectReasonKind::Swearing),
            RawEffectReasonKind::Hater => Some(EffectReasonKind::Hater),
            RawEffectReasonKind::Uncultured => Some(EffectReasonKind::Uncultured),
            RawEffectReasonKind::Unknown(unknown) => Err(Error::UnknownVariant(unknown))?,
        })
    }
}
