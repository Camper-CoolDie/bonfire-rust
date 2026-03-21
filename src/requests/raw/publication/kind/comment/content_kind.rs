use std::result::Result as StdResult;

use serde::Deserialize;

pub(crate) enum RawContentKind {
    Text,
    Image,
    Gif,
    Images,
    Sticker,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawContentKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            0 => RawContentKind::Text,
            3 => RawContentKind::Image,
            4 => RawContentKind::Gif,
            5 => RawContentKind::Images,
            6 => RawContentKind::Sticker,
            other => RawContentKind::Unknown(other),
        })
    }
}
