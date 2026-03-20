use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::account::EffectKind;
use crate::{Error, Result};

pub(crate) enum RawEffectKind {
    Hater,
    Pig,
    Watchman,
    Goose,
    EternalWinter,
    Punished,
    Translator,
    MentionLock,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawEffectKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawEffectKind::Hater,
            2 => RawEffectKind::Pig,
            3 => RawEffectKind::Watchman,
            4 => RawEffectKind::Goose,
            5 => RawEffectKind::EternalWinter,
            6 => RawEffectKind::Punished,
            7 => RawEffectKind::Translator,
            8 => RawEffectKind::MentionLock,
            other => RawEffectKind::Unknown(other),
        })
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
            RawEffectKind::Unknown(unknown) => Err(Error::UnknownVariant(unknown))?,
        })
    }
}
