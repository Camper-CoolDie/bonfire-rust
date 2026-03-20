mod any;
mod post;
mod post_tag;

pub(crate) use any::AnyRawPublication;
pub(crate) use post::RawPost;
pub(crate) use post_tag::RawPostTag;

use crate::models::publication::PublicationKind;

pub(crate) enum RawPublicationKind {
    Unknown(i64),
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
}

impl From<i64> for RawPublicationKind {
    fn from(value: i64) -> Self {
        match value {
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
        }
    }
}

impl From<RawPublicationKind> for PublicationKind {
    fn from(value: RawPublicationKind) -> Self {
        match value {
            RawPublicationKind::Unknown(unknown) => PublicationKind::Unknown(unknown),
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

impl From<RawPublicationKind> for Option<PublicationKind> {
    fn from(value: RawPublicationKind) -> Self {
        match value {
            RawPublicationKind::Unknown(0) => None,
            RawPublicationKind::Unknown(unknown) => Some(PublicationKind::Unknown(unknown)),
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
        }
    }
}
impl From<PublicationKind> for i64 {
    fn from(value: PublicationKind) -> Self {
        match value {
            PublicationKind::Unknown(unknown) => unknown,
            PublicationKind::Comment => 1,
            PublicationKind::ChatMessage => 8,
            PublicationKind::Post => 9,
            PublicationKind::PostTag => 10,
            PublicationKind::Moderation => 11,
            PublicationKind::UserEvent => 12,
            PublicationKind::StickerPack => 15,
            PublicationKind::Sticker => 16,
            PublicationKind::ModerationEvent => 17,
            PublicationKind::AdminEvent => 18,
            PublicationKind::FandomEvent => 19,
            PublicationKind::Quest => 21,
        }
    }
}
