use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::account::EffectReasonKind;

#[derive(Debug)]
pub(crate) enum RawReasonKind {
    Gods,
    RejectedBlocks,
    TooManyBlocks,
    Swearing,
    Hater,
    Uncultured,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawReasonKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawReasonKind::Gods,
            2 => RawReasonKind::RejectedBlocks,
            3 => RawReasonKind::TooManyBlocks,
            4 => RawReasonKind::Swearing,
            5 => RawReasonKind::Hater,
            6 => RawReasonKind::Uncultured,
            other => RawReasonKind::Unknown(other),
        })
    }
}

impl From<RawReasonKind> for EffectReasonKind {
    fn from(value: RawReasonKind) -> Self {
        match value {
            RawReasonKind::Gods => EffectReasonKind::Gods,
            RawReasonKind::RejectedBlocks => EffectReasonKind::RejectedBlocks,
            RawReasonKind::TooManyBlocks => EffectReasonKind::TooManyBlocks,
            RawReasonKind::Swearing => EffectReasonKind::Swearing,
            RawReasonKind::Hater => EffectReasonKind::Hater,
            RawReasonKind::Uncultured => EffectReasonKind::Uncultured,
            RawReasonKind::Unknown(kind) => EffectReasonKind::Unknown(kind),
        }
    }
}
