use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::publication::Status;
use crate::{Error, Result};

#[derive(Debug)]
pub(crate) enum RawStatus {
    Draft,
    Published,
    Blocked,
    DeepBlocked,
    Pending,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawStatus {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawStatus::Draft,
            2 => RawStatus::Published,
            3 => RawStatus::Blocked,
            4 => RawStatus::DeepBlocked,
            5 => RawStatus::Pending,
            other => RawStatus::Unknown(other),
        })
    }
}

impl TryFrom<RawStatus> for Option<Status> {
    type Error = Error;

    fn try_from(value: RawStatus) -> Result<Self> {
        Ok(match value {
            RawStatus::Unknown(0) => None,
            RawStatus::Draft => Some(Status::Draft),
            RawStatus::Published => Some(Status::Published),
            RawStatus::Blocked => Some(Status::Blocked),
            RawStatus::DeepBlocked => Some(Status::DeepBlocked),
            RawStatus::Pending => Some(Status::Pending),
            RawStatus::Unknown(_) => return Err(Error::UnknownVariant(Box::new(value))),
        })
    }
}
