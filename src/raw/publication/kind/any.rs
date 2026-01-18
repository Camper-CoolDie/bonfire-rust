use serde_json::Value;

use crate::models::AnyPublication;
use crate::raw::publication::{RawPublicationInheritor, RawPublicationKind};
use crate::raw::RawPost;
use crate::Result;

pub(crate) enum AnyRawPublication {
    Unknown,
    Comment,
    ChatMessage,
    Post(RawPost),
    PostTag,
    Moderation,
    UserEvent,
    StickerPack,
    Sticker,
    ModerationEvent,
    AdminEvent,
    FandomEvent,
    Quest,
}

impl RawPublicationInheritor for AnyRawPublication {
    type Target = AnyPublication;

    fn new(data: Value, id: u64, kind: RawPublicationKind) -> Result<Self> {
        Ok(match kind {
            RawPublicationKind::Unknown => AnyRawPublication::Unknown,
            RawPublicationKind::Comment => AnyRawPublication::Comment,
            RawPublicationKind::ChatMessage => AnyRawPublication::ChatMessage,
            RawPublicationKind::Post => AnyRawPublication::Post(RawPost::new(data, id, kind)?),
            RawPublicationKind::PostTag => AnyRawPublication::PostTag,
            RawPublicationKind::Moderation => AnyRawPublication::Moderation,
            RawPublicationKind::UserEvent => AnyRawPublication::UserEvent,
            RawPublicationKind::StickerPack => AnyRawPublication::StickerPack,
            RawPublicationKind::Sticker => AnyRawPublication::Sticker,
            RawPublicationKind::ModerationEvent => AnyRawPublication::ModerationEvent,
            RawPublicationKind::AdminEvent => AnyRawPublication::AdminEvent,
            RawPublicationKind::FandomEvent => AnyRawPublication::FandomEvent,
            RawPublicationKind::Quest => AnyRawPublication::Quest,
        })
    }
}

impl TryFrom<AnyRawPublication> for AnyPublication {
    type Error = crate::Error;

    fn try_from(value: AnyRawPublication) -> Result<Self> {
        Ok(match value {
            AnyRawPublication::Unknown => AnyPublication::Unknown,
            AnyRawPublication::Comment => AnyPublication::Comment,
            AnyRawPublication::ChatMessage => AnyPublication::ChatMessage,
            AnyRawPublication::Post(post) => AnyPublication::Post(post.try_into()?),
            AnyRawPublication::PostTag => AnyPublication::PostTag,
            AnyRawPublication::Moderation => AnyPublication::Moderation,
            AnyRawPublication::UserEvent => AnyPublication::UserEvent,
            AnyRawPublication::StickerPack => AnyPublication::StickerPack,
            AnyRawPublication::Sticker => AnyPublication::Sticker,
            AnyRawPublication::ModerationEvent => AnyPublication::ModerationEvent,
            AnyRawPublication::AdminEvent => AnyPublication::AdminEvent,
            AnyRawPublication::FandomEvent => AnyPublication::FandomEvent,
            AnyRawPublication::Quest => AnyPublication::Quest,
        })
    }
}
