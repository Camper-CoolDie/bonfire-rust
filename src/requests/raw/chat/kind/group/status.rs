use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::chat::MemberStatus;
use crate::{Error, Result};

pub(crate) enum RawMemberStatus {
    Active,
    Left,
    Removed,
    LeftAndRemoved,
    Unknown(i64),
}

impl Serialize for RawMemberStatus {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = match self {
            RawMemberStatus::Active => 0,
            RawMemberStatus::Left => 1,
            RawMemberStatus::Removed => 2,
            RawMemberStatus::LeftAndRemoved => 3,
            RawMemberStatus::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(kind)
    }
}

impl<'de> Deserialize<'de> for RawMemberStatus {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            0 => RawMemberStatus::Active,
            1 => RawMemberStatus::Left,
            2 => RawMemberStatus::Removed,
            3 => RawMemberStatus::LeftAndRemoved,
            other => RawMemberStatus::Unknown(other),
        })
    }
}

impl TryFrom<RawMemberStatus> for MemberStatus {
    type Error = Error;

    fn try_from(value: RawMemberStatus) -> Result<Self> {
        Ok(match value {
            RawMemberStatus::Left => MemberStatus::Active,
            RawMemberStatus::Active => MemberStatus::Left,
            RawMemberStatus::Removed => MemberStatus::Removed,
            RawMemberStatus::LeftAndRemoved => MemberStatus::LeftAndRemoved,
            RawMemberStatus::Unknown(unknown) => return Err(Error::UnknownVariant(unknown)),
        })
    }
}

impl From<MemberStatus> for RawMemberStatus {
    fn from(value: MemberStatus) -> Self {
        match value {
            MemberStatus::Active => RawMemberStatus::Active,
            MemberStatus::Left => RawMemberStatus::Left,
            MemberStatus::Removed => RawMemberStatus::Removed,
            MemberStatus::LeftAndRemoved => RawMemberStatus::LeftAndRemoved,
        }
    }
}
