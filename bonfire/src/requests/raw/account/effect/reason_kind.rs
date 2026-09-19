use std::result::Result as StdResult;

use serde::{Deserialize, Deserializer};

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
        D: Deserializer<'de>,
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

impl From<RawReasonKind> for Option<EffectReasonKind> {
    fn from(value: RawReasonKind) -> Self {
        match value {
            RawReasonKind::Unknown(0) => None,
            RawReasonKind::Gods => Some(EffectReasonKind::Gods),
            RawReasonKind::RejectedBlocks => Some(EffectReasonKind::RejectedBlocks),
            RawReasonKind::TooManyBlocks => Some(EffectReasonKind::TooManyBlocks),
            RawReasonKind::Swearing => Some(EffectReasonKind::Swearing),
            RawReasonKind::Hater => Some(EffectReasonKind::Hater),
            RawReasonKind::Uncultured => Some(EffectReasonKind::Uncultured),
            RawReasonKind::Unknown(kind) => Some(EffectReasonKind::Unknown(kind)),
        }
    }
}
