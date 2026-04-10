#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::publication::{Kind, Publishable};
use crate::models::{ChatMessage, Comment, Post, PostTag};
use crate::sealed::Sealed;

/// Represents a union of all possible additional data types for a publication.
///
/// This enum acts as a catch-all for various publication kinds when the specific type is not
/// known or needed, storing additional data relevant to that type. Large variants are
/// [`Box`]-ed to prevent the enum from becoming catastrophically large.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AnyPublication {
    /// The publication contains additional post data
    Post(Box<Post>),
    /// The publication contains additional post tag data
    PostTag(Box<PostTag>),
    /// The publication contains additional comment data
    Comment(Box<Comment>),
    /// The publication contains additional chat message data
    ChatMessage(Box<ChatMessage>),
    /// The publication contains additional moderation data
    Moderation,
    /// The publication contains additional user event data
    UserEvent,
    /// The publication contains additional sticker pack data
    StickerPack,
    /// The publication contains additional sticker data
    Sticker,
    /// The publication contains additional moderation event data
    ModerationEvent,
    /// The publication contains additional admin event data
    AdminEvent,
    /// The publication contains additional fandom event data
    FandomEvent,
    /// The publication contains additional quest data
    Quest,
    /// The publication has an unknown type, thus no specific data can be parsed
    Unknown(i64),
}

impl Publishable for AnyPublication {
    fn to_kind(&self) -> Kind {
        match self {
            AnyPublication::Post(_) => Kind::Post,
            AnyPublication::PostTag(_) => Kind::PostTag,
            AnyPublication::Comment(_) => Kind::Comment,
            AnyPublication::ChatMessage(_) => Kind::ChatMessage,
            AnyPublication::Moderation => Kind::Moderation,
            AnyPublication::UserEvent => Kind::UserEvent,
            AnyPublication::StickerPack => Kind::StickerPack,
            AnyPublication::Sticker => Kind::Sticker,
            AnyPublication::ModerationEvent => Kind::ModerationEvent,
            AnyPublication::AdminEvent => Kind::AdminEvent,
            AnyPublication::FandomEvent => Kind::FandomEvent,
            AnyPublication::Quest => Kind::Quest,
            AnyPublication::Unknown(kind) => Kind::Unknown(*kind),
        }
    }
}

impl Sealed for AnyPublication {}

impl Default for AnyPublication {
    fn default() -> Self {
        Self::Post(Box::default())
    }
}
