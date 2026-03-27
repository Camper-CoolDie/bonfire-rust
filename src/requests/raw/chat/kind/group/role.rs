use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::chat::MemberRole;
use crate::{Error, Result};

#[derive(Debug)]
pub(crate) enum RawMemberRole {
    User,
    Moderator,
    Admin,
    Unknown(i64),
}

impl Serialize for RawMemberRole {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = match self {
            RawMemberRole::User => 1,
            RawMemberRole::Moderator => 2,
            RawMemberRole::Admin => 3,
            RawMemberRole::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(kind)
    }
}

impl<'de> Deserialize<'de> for RawMemberRole {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawMemberRole::User,
            2 => RawMemberRole::Moderator,
            3 => RawMemberRole::Admin,
            other => RawMemberRole::Unknown(other),
        })
    }
}

impl TryFrom<RawMemberRole> for MemberRole {
    type Error = Error;

    fn try_from(value: RawMemberRole) -> Result<Self> {
        Ok(match value {
            RawMemberRole::User => MemberRole::User,
            RawMemberRole::Moderator => MemberRole::Moderator,
            RawMemberRole::Admin => MemberRole::Admin,
            RawMemberRole::Unknown(_) => return Err(Error::UnknownVariant(Box::new(value))),
        })
    }
}

impl From<MemberRole> for RawMemberRole {
    fn from(value: MemberRole) -> Self {
        match value {
            MemberRole::User => RawMemberRole::User,
            MemberRole::Moderator => RawMemberRole::Moderator,
            MemberRole::Admin => RawMemberRole::Admin,
        }
    }
}
