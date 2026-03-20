use serde::de::Error as _;

use crate::models::account::EffectReasonKind;
use crate::{Error, Result};

pub(crate) enum RawEffectReasonKind {
    Unknown(i64),
    Gods,
    RejectedBlocks,
    TooManyBlocks,
    Swearing,
    Hater,
    Uncultured,
}

impl From<i64> for RawEffectReasonKind {
    fn from(value: i64) -> Self {
        match value {
            1 => RawEffectReasonKind::Gods,
            2 => RawEffectReasonKind::RejectedBlocks,
            3 => RawEffectReasonKind::TooManyBlocks,
            4 => RawEffectReasonKind::Swearing,
            5 => RawEffectReasonKind::Hater,
            6 => RawEffectReasonKind::Uncultured,
            other => RawEffectReasonKind::Unknown(other),
        }
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
            RawEffectReasonKind::Unknown(unknown) => Err(serde_json::Error::custom(format!(
                "invalid value: {}, expected one of: {}",
                unknown,
                (0..=6)
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )))?,
        })
    }
}
