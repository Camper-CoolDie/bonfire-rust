mod params;
mod role;
mod status;

pub(crate) use params::RawParams;
pub(crate) use role::RawMemberRole;
use serde::Deserialize;
use serde_json::Value;
pub(crate) use status::RawMemberStatus;

use crate::models::Group;
use crate::requests::raw::chat::{RawKind, RawMessageable};
use crate::requests::raw::{RawChatTag, RawImageRef, timestamp_from_millis};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawGroup {
    #[serde(skip)]
    pub id: u64,
    #[serde(rename = "customName")]
    pub name: String,
    #[serde(rename = "customImage")]
    pub icon: RawImageRef,
    #[serde(rename = "backgroundImage")]
    pub background: RawImageRef,
    #[serde(rename = "memberStatus")]
    pub my_status: RawMemberStatus,
    #[serde(rename = "subscribed")]
    pub is_subscribed: bool,
    #[serde(rename = "membersCount")]
    pub subscribers_count: u64,
    #[serde(rename = "exitDate")]
    pub left_at: i64,
    pub params: RawParams,
}

impl RawMessageable for RawGroup {
    type Target = Group;

    fn new(data: Value, tag: RawChatTag) -> Result<Self> {
        if let RawChatTag::Group { id } = tag {
            let mut fandom_sub = serde_json::from_value::<RawGroup>(data)?;
            fandom_sub.id = id;
            Ok(fandom_sub)
        } else {
            let kind = RawKind::from(tag);
            Err(Error::UnknownVariant(Box::new(kind)))
        }
    }
}

impl TryFrom<RawGroup> for Group {
    type Error = Error;

    fn try_from(value: RawGroup) -> Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
            icon: value.icon.into(),
            background: value.background.into(),
            my_status: value.my_status.try_into()?,
            is_subscribed: value.is_subscribed,
            subscribers_count: value.subscribers_count,
            left_at: match value.left_at {
                0 => None,
                timestamp => Some(timestamp_from_millis(timestamp)?),
            },
            is_public: value.params.is_public,
            allow_invites: value.params.allow_invites,
            allow_changes: value.params.allow_changes,
        })
    }
}
