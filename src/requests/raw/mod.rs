pub(super) mod account;
pub(super) mod chat;
pub(super) mod common;
pub(super) mod error;
pub(super) mod fandom;
pub(super) mod profile;
pub(super) mod publication;

use std::result::Result as StdResult;

pub(super) use account::{
    RawAccount, RawBadge, RawEffect, RawInfo as RawAccountInfo, RawStat as RawAccountStat,
};
pub(super) use chat::{
    AnyRawChat, RawChat, RawDirect, RawFandomRoot, RawFandomSub, RawGroup, RawTag as RawChatTag,
};
use chrono::{DateTime, Utc};
pub(super) use common::{RawCategory, RawImageRef, RawLanguage};
pub(crate) use error::RawRootError;
pub(super) use error::RawUnavailableError;
pub(super) use fandom::RawFandom;
pub(super) use profile::{RawGender, RawLink};
pub(super) use publication::{
    AnyRawPublication, RawChatMessage, RawComment, RawPost, RawPostTag, RawPublication, RawReaction,
};
use serde::de::Error as _;

fn timestamp_from_millis(millis: i64) -> StdResult<DateTime<Utc>, serde_json::Error> {
    DateTime::from_timestamp_millis(millis)
        .ok_or_else(|| serde_json::Error::custom(format!("timestamp {millis} is out of range")))
}
