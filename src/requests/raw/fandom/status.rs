use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::fandom::Status;
use crate::{Error, Result};

#[derive(Debug)]
pub(crate) enum RawStatus {
    Suggested,
    Accepted,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawStatus {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawStatus::Suggested,
            2 => RawStatus::Accepted,
            other => RawStatus::Unknown(other),
        })
    }
}

impl TryFrom<RawStatus> for Option<Status> {
    type Error = Error;

    fn try_from(value: RawStatus) -> Result<Self> {
        Ok(match value {
            RawStatus::Unknown(0) => None,
            RawStatus::Suggested => Some(Status::Suggested),
            RawStatus::Accepted => Some(Status::Accepted),
            RawStatus::Unknown(_) => return Err(Error::UnknownVariant(Box::new(value))),
        })
    }
}
