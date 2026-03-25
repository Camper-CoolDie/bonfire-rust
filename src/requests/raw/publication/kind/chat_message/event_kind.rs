use std::result::Result as StdResult;

use chrono::DateTime;
use serde::Deserialize;
use serde::de::Error as _;

use crate::models::publication::ChatMessageContent;
use crate::requests::raw::RawGender;
use crate::requests::raw::chat::RawMemberRole;
use crate::{Error, Result};

pub(crate) enum RawEventKind {
    Block,
    Create,
    AddMember,
    RemoveMember,
    ChangeIcon,
    Rename,
    Leave,
    Enter,
    ChangeParams,
    ChangeRole,
    ChangeBackground,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawEventKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawEventKind::Block,
            2 => RawEventKind::Create,
            3 => RawEventKind::AddMember,
            4 => RawEventKind::RemoveMember,
            5 => RawEventKind::ChangeIcon,
            6 => RawEventKind::Rename,
            7 => RawEventKind::Leave,
            8 => RawEventKind::Enter,
            9 => RawEventKind::ChangeParams,
            10 => RawEventKind::ChangeRole,
            11 => RawEventKind::ChangeBackground,
            other => RawEventKind::Unknown(other),
        })
    }
}

pub(super) struct IntoEventOptions {
    pub event_kind: RawEventKind,
    pub by_account_id: u64,
    pub by_account_name: String,
    pub by_account_gender: RawGender,
    pub target_name: String,
    pub target_id: u64,
    pub reason: String,
    pub moderation_id: u64,
    pub banned_until: i64,
    pub new_role: RawMemberRole,
}

impl TryFrom<IntoEventOptions> for ChatMessageContent {
    type Error = Error;

    fn try_from(value: IntoEventOptions) -> Result<Self> {
        Ok(match value.event_kind {
            RawEventKind::Block => ChatMessageContent::BlockEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
                name: value.target_name,
                reason: value.reason,
                moderation_id: value.moderation_id,
                is_punished: value.banned_until != 0,
                banned_until: match value.banned_until {
                    // -1: warn, 0: do nothing
                    -1 | 0 => None,
                    timestamp => {
                        Some(DateTime::from_timestamp_millis(timestamp).ok_or_else(|| {
                            serde_json::Error::custom(format!(
                                "timestamp {timestamp} is out of range"
                            ))
                        })?)
                    }
                },
            },
            RawEventKind::Create => ChatMessageContent::CreateEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
            },
            RawEventKind::AddMember => ChatMessageContent::AddMemberEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
                name: value.target_name,
            },
            RawEventKind::RemoveMember => ChatMessageContent::RemoveMemberEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
                name: value.target_name,
            },
            RawEventKind::ChangeRole => ChatMessageContent::ChangeRoleEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
                name: value.target_name,
                new_role: value.new_role.try_into()?,
            },
            RawEventKind::Enter => ChatMessageContent::EnterEvent {
                id: value.by_account_id,
                name: value.by_account_name,
                gender: value.by_account_gender.try_into()?,
            },
            RawEventKind::Leave => ChatMessageContent::LeaveEvent {
                id: value.by_account_id,
                name: value.by_account_name,
                gender: value.by_account_gender.try_into()?,
            },
            RawEventKind::Rename => ChatMessageContent::RenameEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
                new_name: value.target_name,
            },
            RawEventKind::ChangeIcon => ChatMessageContent::ChangeIconEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
                image_id: value.target_id,
            },
            RawEventKind::ChangeBackground => ChatMessageContent::ChangeBackgroundEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
                image_id: match value.target_id {
                    0 => None,
                    id => Some(id),
                },
            },
            RawEventKind::ChangeParams => ChatMessageContent::ChangeParamsEvent {
                by_account_id: value.by_account_id,
                by_account_name: value.by_account_name,
                by_account_gender: value.by_account_gender.try_into()?,
            },
            RawEventKind::Unknown(unknown) => ChatMessageContent::UnknownEvent(unknown),
        })
    }
}
