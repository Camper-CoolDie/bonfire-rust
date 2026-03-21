mod any;
mod comment;
mod post;
mod post_tag;

use std::result::Result as StdResult;

pub(crate) use any::AnyRawPublication;
pub(crate) use comment::{RawComment, RawContentKind as RawCommentContentKind};
pub(crate) use post::RawPost;
pub(crate) use post_tag::RawPostTag;
use serde::{Deserialize, Serialize};

use crate::models::publication::Kind;

pub(crate) enum RawKind {
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

impl Serialize for RawKind {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind = match self {
            RawKind::Comment => 1,
            RawKind::ChatMessage => 8,
            RawKind::Post => 9,
            RawKind::PostTag => 10,
            RawKind::Moderation => 11,
            RawKind::UserEvent => 12,
            RawKind::StickerPack => 15,
            RawKind::Sticker => 16,
            RawKind::ModerationEvent => 17,
            RawKind::AdminEvent => 18,
            RawKind::FandomEvent => 19,
            RawKind::Quest => 21,
            RawKind::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(kind)
    }
}

impl<'de> Deserialize<'de> for RawKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawKind::Comment,
            8 => RawKind::ChatMessage,
            9 => RawKind::Post,
            10 => RawKind::PostTag,
            11 => RawKind::Moderation,
            12 => RawKind::UserEvent,
            15 => RawKind::StickerPack,
            16 => RawKind::Sticker,
            17 => RawKind::ModerationEvent,
            18 => RawKind::AdminEvent,
            19 => RawKind::FandomEvent,
            21 => RawKind::Quest,
            other => RawKind::Unknown(other),
        })
    }
}

impl From<RawKind> for Kind {
    fn from(value: RawKind) -> Self {
        match value {
            RawKind::Comment => Kind::Comment,
            RawKind::ChatMessage => Kind::ChatMessage,
            RawKind::Post => Kind::Post,
            RawKind::PostTag => Kind::PostTag,
            RawKind::Moderation => Kind::Moderation,
            RawKind::UserEvent => Kind::UserEvent,
            RawKind::StickerPack => Kind::StickerPack,
            RawKind::Sticker => Kind::Sticker,
            RawKind::ModerationEvent => Kind::ModerationEvent,
            RawKind::AdminEvent => Kind::AdminEvent,
            RawKind::FandomEvent => Kind::FandomEvent,
            RawKind::Quest => Kind::Quest,
            RawKind::Unknown(unknown) => Kind::Unknown(unknown),
        }
    }
}

impl From<RawKind> for Option<Kind> {
    fn from(value: RawKind) -> Self {
        match value {
            RawKind::Unknown(0) => None,
            RawKind::Comment => Some(Kind::Comment),
            RawKind::ChatMessage => Some(Kind::ChatMessage),
            RawKind::Post => Some(Kind::Post),
            RawKind::PostTag => Some(Kind::PostTag),
            RawKind::Moderation => Some(Kind::Moderation),
            RawKind::UserEvent => Some(Kind::UserEvent),
            RawKind::StickerPack => Some(Kind::StickerPack),
            RawKind::Sticker => Some(Kind::Sticker),
            RawKind::ModerationEvent => Some(Kind::ModerationEvent),
            RawKind::AdminEvent => Some(Kind::AdminEvent),
            RawKind::FandomEvent => Some(Kind::FandomEvent),
            RawKind::Quest => Some(Kind::Quest),
            RawKind::Unknown(unknown) => Some(Kind::Unknown(unknown)),
        }
    }
}

impl From<Kind> for RawKind {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Comment => RawKind::Comment,
            Kind::ChatMessage => RawKind::ChatMessage,
            Kind::Post => RawKind::Post,
            Kind::PostTag => RawKind::PostTag,
            Kind::Moderation => RawKind::Moderation,
            Kind::UserEvent => RawKind::UserEvent,
            Kind::StickerPack => RawKind::StickerPack,
            Kind::Sticker => RawKind::Sticker,
            Kind::ModerationEvent => RawKind::ModerationEvent,
            Kind::AdminEvent => RawKind::AdminEvent,
            Kind::FandomEvent => RawKind::FandomEvent,
            Kind::Quest => RawKind::Quest,
            Kind::Unknown(unknown) => RawKind::Unknown(unknown),
        }
    }
}
