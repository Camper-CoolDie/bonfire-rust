use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::settings::FeedKind;
use crate::{Error, Result};

#[derive(Debug)]
pub(crate) enum RawKind {
    Follows,
    All,
    Best,
    Good,
    Abyss,
    AllWithFollows,
    Unknown(i64),
}

impl Serialize for RawKind {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = match self {
            RawKind::Follows => 1,
            RawKind::All => 2,
            RawKind::Best => 3,
            RawKind::Good => 4,
            RawKind::Abyss => 5,
            RawKind::AllWithFollows => 6,
            RawKind::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(kind)
    }
}

impl<'de> Deserialize<'de> for RawKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawKind::Follows,
            2 => RawKind::All,
            3 => RawKind::Best,
            4 => RawKind::Good,
            5 => RawKind::Abyss,
            6 => RawKind::AllWithFollows,
            other => RawKind::Unknown(other),
        })
    }
}

impl TryFrom<RawKind> for FeedKind {
    type Error = Error;

    fn try_from(value: RawKind) -> Result<Self> {
        Ok(match value {
            RawKind::Follows => FeedKind::Follows,
            RawKind::All => FeedKind::All,
            RawKind::Best => FeedKind::Best,
            RawKind::Good => FeedKind::Good,
            RawKind::Abyss => FeedKind::Abyss,
            RawKind::AllWithFollows => FeedKind::AllWithFollows,
            RawKind::Unknown(_) => return Err(Error::UnknownVariant(Box::new(value))),
        })
    }
}

impl From<FeedKind> for RawKind {
    fn from(value: FeedKind) -> Self {
        match value {
            FeedKind::Follows => RawKind::Follows,
            FeedKind::All => RawKind::All,
            FeedKind::Best => RawKind::Best,
            FeedKind::Good => RawKind::Good,
            FeedKind::Abyss => RawKind::Abyss,
            FeedKind::AllWithFollows => RawKind::AllWithFollows,
        }
    }
}
