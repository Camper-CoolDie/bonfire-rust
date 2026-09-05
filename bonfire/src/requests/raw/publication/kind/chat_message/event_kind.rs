use std::result::Result as StdResult;

use serde::{Deserialize, Deserializer};

use crate::models::publication::ChatMessageContent;
use crate::requests::raw::chat::RawMemberRole;
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::requests::raw::{RawAccountRef, RawGender};
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
        D: Deserializer<'de>,
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
    pub kind: RawEventKind,
    pub creator_id: u64,
    pub creator_name: String,
    pub creator_gender: RawGender,
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
        let creator = RawAccountRef {
            id: value.creator_id,
            name: value.creator_name,
            gender: value.creator_gender,
        };

        Ok(match value.kind {
            RawEventKind::Block => ChatMessageContent::BlockEvent {
                target_name: value.target_name,
                moderation_id: value.moderation_id,
                moderator: creator.try_into()?,
                is_punished: value.banned_until != 0,
                banned_until: match value.banned_until {
                    // -1: warn, 0: do nothing
                    -1 | 0 => None,
                    timestamp => Some(timestamp_from_millis(timestamp)?),
                },
                reason: value.reason,
            },
            RawEventKind::Create => ChatMessageContent::CreateEvent {
                moderator: creator.try_into()?,
            },
            RawEventKind::AddMember => ChatMessageContent::AddMemberEvent {
                target_name: value.target_name,
                member: creator.try_into()?,
            },
            RawEventKind::RemoveMember => ChatMessageContent::RemoveMemberEvent {
                target_name: value.target_name,
                member: creator.try_into()?,
            },
            RawEventKind::ChangeRole => ChatMessageContent::ChangeRoleEvent {
                target_name: value.target_name,
                new_role: value.new_role.try_into()?,
                member: creator.try_into()?,
            },
            RawEventKind::Enter => ChatMessageContent::EnterEvent(creator.try_into()?),
            RawEventKind::Leave => ChatMessageContent::LeaveEvent(creator.try_into()?),
            RawEventKind::Rename => ChatMessageContent::RenameEvent {
                new_name: value.target_name,
                member: creator.try_into()?,
            },
            RawEventKind::ChangeIcon => ChatMessageContent::ChangeIconEvent {
                new_icon_id: value.target_id,
                member: creator.try_into()?,
            },
            RawEventKind::ChangeBackground => ChatMessageContent::ChangeBackgroundEvent {
                new_background_id: match value.target_id {
                    0 => None,
                    id => Some(id),
                },
                member: creator.try_into()?,
            },
            RawEventKind::ChangeParams => ChatMessageContent::ChangeParamsEvent {
                member: creator.try_into()?,
            },
            RawEventKind::Unknown(unknown) => ChatMessageContent::UnknownEvent(unknown),
        })
    }
}
