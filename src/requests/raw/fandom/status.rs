use serde::de::Error as _;

use crate::models::fandom::FandomStatus;
use crate::{Error, Result};

pub(crate) enum RawFandomStatus {
    Unknown(i64),
    Suggested,
    Accepted,
}

impl From<i64> for RawFandomStatus {
    fn from(value: i64) -> Self {
        match value {
            1 => RawFandomStatus::Suggested,
            2 => RawFandomStatus::Accepted,
            other => RawFandomStatus::Unknown(other),
        }
    }
}

impl TryFrom<RawFandomStatus> for Option<FandomStatus> {
    type Error = Error;

    fn try_from(value: RawFandomStatus) -> Result<Self> {
        Ok(match value {
            RawFandomStatus::Unknown(0) => None,
            RawFandomStatus::Suggested => Some(FandomStatus::Suggested),
            RawFandomStatus::Accepted => Some(FandomStatus::Accepted),
            RawFandomStatus::Unknown(unknown) => Err(serde_json::Error::custom(format!(
                "invalid value: {}, expected one of: {}",
                unknown,
                (0..=2)
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )))?,
        })
    }
}
