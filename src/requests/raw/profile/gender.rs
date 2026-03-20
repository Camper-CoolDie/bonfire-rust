use serde::de::Error as _;

use crate::models::Gender;
use crate::{Error, Result};

pub(crate) enum RawGender {
    Unknown(i64),
    Male,
    Female,
    Other,
}

impl From<i64> for RawGender {
    fn from(value: i64) -> Self {
        match value {
            0 => RawGender::Male,
            1 => RawGender::Female,
            2 => RawGender::Other,
            other => RawGender::Unknown(other),
        }
    }
}

impl TryFrom<RawGender> for Gender {
    type Error = Error;

    fn try_from(value: RawGender) -> Result<Self> {
        Ok(match value {
            RawGender::Male => Gender::Male,
            RawGender::Female => Gender::Female,
            RawGender::Other => Gender::Other,
            RawGender::Unknown(unknown) => Err(serde_json::Error::custom(format!(
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

impl From<Gender> for i64 {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => 0,
            Gender::Female => 1,
            Gender::Other => 2,
        }
    }
}
