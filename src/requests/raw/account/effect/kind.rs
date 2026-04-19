use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::account::EffectKind;

#[derive(Debug)]
pub(crate) enum RawKind {
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

impl<'de> Deserialize<'de> for RawKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawKind::Hater,
            2 => RawKind::Pig,
            3 => RawKind::Watchman,
            4 => RawKind::Goose,
            5 => RawKind::EternalWinter,
            6 => RawKind::Punished,
            7 => RawKind::Translator,
            8 => RawKind::MentionLock,
            other => RawKind::Unknown(other),
        })
    }
}

impl From<RawKind> for EffectKind {
    fn from(value: RawKind) -> Self {
        match value {
            RawKind::Hater => EffectKind::Hater,
            RawKind::Pig => EffectKind::Pig,
            RawKind::Watchman => EffectKind::Watchman,
            RawKind::Goose => EffectKind::Goose,
            RawKind::EternalWinter => EffectKind::EternalWinter,
            RawKind::Punished => EffectKind::Punished,
            RawKind::Translator => EffectKind::Translator,
            RawKind::MentionLock => EffectKind::MentionLock,
            RawKind::Unknown(kind) => EffectKind::Unknown(kind),
        }
    }
}
