mod any;
mod post;

pub(crate) use any::AnyRawPublication;
pub(crate) use post::RawPost;
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::publication::PublicationKind;

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub(crate) enum RawPublicationKind {
    #[serde(other)]
    Unknown = 20,
    Comment = 1,
    ChatMessage = 8,
    Post = 9,
    PostTag = 10,
    Moderation = 11,
    UserEvent = 12,
    StickerPack = 15,
    Sticker = 16,
    ModerationEvent = 17,
    AdminEvent = 18,
    FandomEvent = 19,
    Quest = 21,
}

impl From<RawPublicationKind> for PublicationKind {
    fn from(value: RawPublicationKind) -> Self {
        match value {
            RawPublicationKind::Unknown => PublicationKind::Unknown,
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
        }
    }
}

impl From<PublicationKind> for RawPublicationKind {
    fn from(value: PublicationKind) -> Self {
        match value {
            PublicationKind::Unknown => RawPublicationKind::Unknown,
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
        }
    }
}
