use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::Gender;
use crate::{Error, Result};

pub(crate) enum RawGender {
    Male,
    Female,
    Other,
    Unknown(i64),
}

impl Serialize for RawGender {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = match self {
            RawGender::Male => 0,
            RawGender::Female => 1,
            RawGender::Other => 2,
            RawGender::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(kind)
    }
}

impl<'de> Deserialize<'de> for RawGender {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            0 => RawGender::Male,
            1 => RawGender::Female,
            2 => RawGender::Other,
            other => RawGender::Unknown(other),
        })
    }
}

impl TryFrom<RawGender> for Gender {
    type Error = Error;

    fn try_from(value: RawGender) -> Result<Self> {
        Ok(match value {
            RawGender::Male => Gender::Male,
            RawGender::Female => Gender::Female,
            RawGender::Other => Gender::Other,
            RawGender::Unknown(unknown) => Err(Error::UnknownVariant(unknown))?,
        })
    }
}

impl From<Gender> for RawGender {
    fn from(value: Gender) -> Self {
        match value {
            Gender::Male => RawGender::Male,
            Gender::Female => RawGender::Female,
            Gender::Other => RawGender::Other,
        }
    }
}
