mod any;
mod post;
mod post_tag;

use std::result::Result as StdResult;

pub(crate) use any::AnyRawPublication;
pub(crate) use post::RawPost;
pub(crate) use post_tag::RawPostTag;
use serde::{Deserialize, Serialize};

use crate::models::publication::PublicationKind;

pub(crate) enum RawPublicationKind {
    Comment,
    ChatMessage,
    Post,
    PostTag,
    Moderation,
    UserEvent,
    StickerPack,
    Sticker,
    ModerationEvent,
    AdminEvent,
    FandomEvent,
    Quest,
    Unknown(i64),
}

impl Serialize for RawPublicationKind {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = match self {
            RawPublicationKind::Comment => 1,
            RawPublicationKind::ChatMessage => 8,
            RawPublicationKind::Post => 9,
            RawPublicationKind::PostTag => 10,
            RawPublicationKind::Moderation => 11,
            RawPublicationKind::UserEvent => 12,
            RawPublicationKind::StickerPack => 15,
            RawPublicationKind::Sticker => 16,
            RawPublicationKind::ModerationEvent => 17,
            RawPublicationKind::AdminEvent => 18,
            RawPublicationKind::FandomEvent => 19,
            RawPublicationKind::Quest => 21,
            RawPublicationKind::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(kind)
    }
}

impl<'de> Deserialize<'de> for RawPublicationKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawPublicationKind::Comment,
            8 => RawPublicationKind::ChatMessage,
            9 => RawPublicationKind::Post,
            10 => RawPublicationKind::PostTag,
            11 => RawPublicationKind::Moderation,
            12 => RawPublicationKind::UserEvent,
            15 => RawPublicationKind::StickerPack,
            16 => RawPublicationKind::Sticker,
            17 => RawPublicationKind::ModerationEvent,
            18 => RawPublicationKind::AdminEvent,
            19 => RawPublicationKind::FandomEvent,
            21 => RawPublicationKind::Quest,
            other => RawPublicationKind::Unknown(other),
        })
    }
}

impl From<RawPublicationKind> for PublicationKind {
    fn from(value: RawPublicationKind) -> Self {
        match value {
            RawPublicationKind::Comment => PublicationKind::Comment,
            RawPublicationKind::ChatMessage => PublicationKind::ChatMessage,
            RawPublicationKind::Post => PublicationKind::Post,
            RawPublicationKind::PostTag => PublicationKind::PostTag,
            RawPublicationKind::Moderation => PublicationKind::Moderation,
            RawPublicationKind::UserEvent => PublicationKind::UserEvent,
            RawPublicationKind::StickerPack => PublicationKind::StickerPack,
            RawPublicationKind::Sticker => PublicationKind::Sticker,
            RawPublicationKind::ModerationEvent => PublicationKind::ModerationEvent,
            RawPublicationKind::AdminEvent => PublicationKind::AdminEvent,
            RawPublicationKind::FandomEvent => PublicationKind::FandomEvent,
            RawPublicationKind::Quest => PublicationKind::Quest,
            RawPublicationKind::Unknown(unknown) => PublicationKind::Unknown(unknown),
        }
    }
}

impl From<RawPublicationKind> for Option<PublicationKind> {
    fn from(value: RawPublicationKind) -> Self {
        match value {
            RawPublicationKind::Unknown(0) => None,
            RawPublicationKind::Comment => Some(PublicationKind::Comment),
            RawPublicationKind::ChatMessage => Some(PublicationKind::ChatMessage),
            RawPublicationKind::Post => Some(PublicationKind::Post),
            RawPublicationKind::PostTag => Some(PublicationKind::PostTag),
            RawPublicationKind::Moderation => Some(PublicationKind::Moderation),
            RawPublicationKind::UserEvent => Some(PublicationKind::UserEvent),
            RawPublicationKind::StickerPack => Some(PublicationKind::StickerPack),
            RawPublicationKind::Sticker => Some(PublicationKind::Sticker),
            RawPublicationKind::ModerationEvent => Some(PublicationKind::ModerationEvent),
            RawPublicationKind::AdminEvent => Some(PublicationKind::AdminEvent),
            RawPublicationKind::FandomEvent => Some(PublicationKind::FandomEvent),
            RawPublicationKind::Quest => Some(PublicationKind::Quest),
            RawPublicationKind::Unknown(unknown) => Some(PublicationKind::Unknown(unknown)),
        }
    }
}

impl From<PublicationKind> for RawPublicationKind {
    fn from(value: PublicationKind) -> Self {
        match value {
            PublicationKind::Comment => RawPublicationKind::Comment,
            PublicationKind::ChatMessage => RawPublicationKind::ChatMessage,
            PublicationKind::Post => RawPublicationKind::Post,
            PublicationKind::PostTag => RawPublicationKind::PostTag,
            PublicationKind::Moderation => RawPublicationKind::Moderation,
            PublicationKind::UserEvent => RawPublicationKind::UserEvent,
            PublicationKind::StickerPack => RawPublicationKind::StickerPack,
            PublicationKind::Sticker => RawPublicationKind::Sticker,
            PublicationKind::ModerationEvent => RawPublicationKind::ModerationEvent,
            PublicationKind::AdminEvent => RawPublicationKind::AdminEvent,
            PublicationKind::FandomEvent => RawPublicationKind::FandomEvent,
            PublicationKind::Quest => RawPublicationKind::Quest,
            PublicationKind::Unknown(unknown) => RawPublicationKind::Unknown(unknown),
        }
    }
}
