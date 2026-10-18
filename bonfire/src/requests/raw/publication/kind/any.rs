use serde_json::Value;

use crate::models::AnyPublication;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::requests::raw::{
    RawChatMessage, RawComment, RawPost, RawPostTag, RawSticker, RawStickerPack,
};
use crate::{Error, Result};

pub(crate) enum AnyRawPublication {
    Post(Box<RawPost>),
    PostTag(Box<RawPostTag>),
    Comment(Box<RawComment>),
    ChatMessage(Box<RawChatMessage>),
    Moderation,
    UserEvent,
    StickerPack(Box<RawStickerPack>),
    Sticker(Box<RawSticker>),
    ModerationEvent,
    AdminEvent,
    FandomEvent,
    Quest,
    Unknown(i64),
}

impl RawPublishable for AnyRawPublication {
    type Target = AnyPublication;

    fn new(data: Value, kind: RawKind) -> Result<Self> {
        Ok(match kind {
            RawKind::Post => AnyRawPublication::Post(Box::new(RawPost::new(data, kind)?)),
            RawKind::PostTag => AnyRawPublication::PostTag(Box::new(RawPostTag::new(data, kind)?)),
            RawKind::Comment => AnyRawPublication::Comment(Box::new(RawComment::new(data, kind)?)),
            RawKind::ChatMessage => {
                AnyRawPublication::ChatMessage(Box::new(RawChatMessage::new(data, kind)?))
            }
            RawKind::Moderation => AnyRawPublication::Moderation,
            RawKind::UserEvent => AnyRawPublication::UserEvent,
            RawKind::StickerPack => {
                AnyRawPublication::StickerPack(Box::new(RawStickerPack::new(data, kind)?))
            }
            RawKind::Sticker => AnyRawPublication::Sticker(Box::new(RawSticker::new(data, kind)?)),
            RawKind::ModerationEvent => AnyRawPublication::ModerationEvent,
            RawKind::AdminEvent => AnyRawPublication::AdminEvent,
            RawKind::FandomEvent => AnyRawPublication::FandomEvent,
            RawKind::Quest => AnyRawPublication::Quest,
            RawKind::Unknown(kind) => AnyRawPublication::Unknown(kind),
        })
    }
}

impl TryFrom<AnyRawPublication> for AnyPublication {
    type Error = Error;

    fn try_from(value: AnyRawPublication) -> Result<Self> {
        Ok(match value {
            AnyRawPublication::Post(post) => AnyPublication::Post(Box::new((*post).try_into()?)),
            AnyRawPublication::PostTag(tag) => {
                AnyPublication::PostTag(Box::new((*tag).try_into()?))
            }
            AnyRawPublication::Comment(comment) => {
                AnyPublication::Comment(Box::new((*comment).try_into()?))
            }
            AnyRawPublication::ChatMessage(message) => {
                AnyPublication::ChatMessage(Box::new((*message).try_into()?))
            }
            AnyRawPublication::Moderation => AnyPublication::Moderation,
            AnyRawPublication::UserEvent => AnyPublication::UserEvent,
            AnyRawPublication::StickerPack(pack) => {
                AnyPublication::StickerPack(Box::new((*pack).try_into()?))
            }
            AnyRawPublication::Sticker(sticker) => {
                AnyPublication::Sticker(Box::new((*sticker).try_into()?))
            }
            AnyRawPublication::ModerationEvent => AnyPublication::ModerationEvent,
            AnyRawPublication::AdminEvent => AnyPublication::AdminEvent,
            AnyRawPublication::FandomEvent => AnyPublication::FandomEvent,
            AnyRawPublication::Quest => AnyPublication::Quest,
            AnyRawPublication::Unknown(kind) => AnyPublication::Unknown(kind),
        })
    }
}
