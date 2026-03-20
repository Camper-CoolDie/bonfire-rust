use serde::de::Error as _;

use crate::models::account::EffectKind;
use crate::{Error, Result};

pub(crate) enum RawEffectKind {
    Unknown(i64),
    Hater,
    Pig,
    Watchman,
    Goose,
    EternalWinter,
    Punished,
    Translator,
    MentionLock,
}

impl From<i64> for RawEffectKind {
    fn from(value: i64) -> Self {
        match value {
            1 => RawEffectKind::Hater,
            2 => RawEffectKind::Pig,
            3 => RawEffectKind::Watchman,
            4 => RawEffectKind::Goose,
            5 => RawEffectKind::EternalWinter,
            6 => RawEffectKind::Punished,
            7 => RawEffectKind::Translator,
            8 => RawEffectKind::MentionLock,
            other => RawEffectKind::Unknown(other),
        }
    }
}

impl TryFrom<RawEffectKind> for EffectKind {
    type Error = Error;

    fn try_from(value: RawEffectKind) -> Result<Self> {
        Ok(match value {
            RawEffectKind::Hater => EffectKind::Hater,
            RawEffectKind::Pig => EffectKind::Pig,
            RawEffectKind::Watchman => EffectKind::Watchman,
            RawEffectKind::Goose => EffectKind::Goose,
            RawEffectKind::EternalWinter => EffectKind::EternalWinter,
            RawEffectKind::Punished => EffectKind::Punished,
            RawEffectKind::Translator => EffectKind::Translator,
            RawEffectKind::MentionLock => EffectKind::MentionLock,
            RawEffectKind::Unknown(unknown) => Err(serde_json::Error::custom(format!(
                "invalid value: {}, expected one of: {}",
                unknown,
                (1..=8)
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )))?,
        })
    }
}
