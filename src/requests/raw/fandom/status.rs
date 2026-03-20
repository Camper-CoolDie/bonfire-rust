use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::fandom::FandomStatus;
use crate::{Error, Result};

pub(crate) enum RawFandomStatus {
    Suggested,
    Accepted,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawFandomStatus {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawFandomStatus::Suggested,
            2 => RawFandomStatus::Accepted,
            other => RawFandomStatus::Unknown(other),
        })
    }
}

impl TryFrom<RawFandomStatus> for Option<FandomStatus> {
    type Error = Error;

    fn try_from(value: RawFandomStatus) -> Result<Self> {
        Ok(match value {
            RawFandomStatus::Unknown(0) => None,
            RawFandomStatus::Suggested => Some(FandomStatus::Suggested),
            RawFandomStatus::Accepted => Some(FandomStatus::Accepted),
            RawFandomStatus::Unknown(unknown) => Err(Error::UnknownVariant(unknown))?,
        })
    }
}
