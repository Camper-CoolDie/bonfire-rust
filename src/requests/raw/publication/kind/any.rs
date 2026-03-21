use serde_json::Value;

use crate::models::AnyPublication;
use crate::requests::raw::publication::{RawComment, RawKind, RawPublishable};
use crate::requests::raw::{RawPost, RawPostTag};
use crate::{Error, Result};

pub(crate) enum AnyRawPublication {
    Comment(Box<RawComment>),
    ChatMessage,
    Post(Box<RawPost>),
    PostTag(Box<RawPostTag>),
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

impl RawPublishable for AnyRawPublication {
    type Target = AnyPublication;

    fn new(data: Value, kind: RawKind) -> Result<Self> {
        Ok(match kind {
            RawKind::Comment => AnyRawPublication::Comment(Box::new(RawComment::new(data, kind)?)),
            RawKind::ChatMessage => AnyRawPublication::ChatMessage,
            RawKind::Post => AnyRawPublication::Post(Box::new(RawPost::new(data, kind)?)),
            RawKind::PostTag => AnyRawPublication::PostTag(Box::new(RawPostTag::new(data, kind)?)),
            RawKind::Moderation => AnyRawPublication::Moderation,
            RawKind::UserEvent => AnyRawPublication::UserEvent,
            RawKind::StickerPack => AnyRawPublication::StickerPack,
            RawKind::Sticker => AnyRawPublication::Sticker,
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
            AnyRawPublication::Comment(comment) => {
                AnyPublication::Comment(Box::new((*comment).try_into()?))
            }
            AnyRawPublication::ChatMessage => AnyPublication::ChatMessage,
            AnyRawPublication::Post(post) => AnyPublication::Post(Box::new((*post).try_into()?)),
            AnyRawPublication::PostTag(post_tag) => {
                AnyPublication::PostTag(Box::new((*post_tag).try_into()?))
            }
            AnyRawPublication::Moderation => AnyPublication::Moderation,
            AnyRawPublication::UserEvent => AnyPublication::UserEvent,
            AnyRawPublication::StickerPack => AnyPublication::StickerPack,
            AnyRawPublication::Sticker => AnyPublication::Sticker,
            AnyRawPublication::ModerationEvent => AnyPublication::ModerationEvent,
            AnyRawPublication::AdminEvent => AnyPublication::AdminEvent,
            AnyRawPublication::FandomEvent => AnyPublication::FandomEvent,
            AnyRawPublication::Quest => AnyPublication::Quest,
            AnyRawPublication::Unknown(kind) => AnyPublication::Unknown(kind),
        })
    }
}
