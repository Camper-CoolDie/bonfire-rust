use crate::models::publication::{PublicationInheritor, PublicationKind};
use crate::models::Post;
use crate::sealed::Sealed;

/// Represents a union of all possible additional data types for a publication.
///
/// This enum acts as a catch-all for various publication kinds when the specific type is not
/// known or needed, storing additional data relevant to that type.
#[derive(Default, Clone, Debug)]
pub enum AnyPublication {
    /// The publication has an unknown or unspecified type, thus no specific data can be parsed
    #[default]
    Unknown,
    /// The publication contains additional comment data
    Comment,
    /// The publication contains additional chat message data
    ChatMessage,
    /// The publication contains additional post data
    Post(Post),
    /// The publication contains additional post tag data
    PostTag,
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
}

impl PublicationInheritor for AnyPublication {
    fn kind(&self) -> PublicationKind {
        match self {
            AnyPublication::Unknown => PublicationKind::Unknown,
            AnyPublication::Comment => PublicationKind::Comment,
            AnyPublication::ChatMessage => PublicationKind::ChatMessage,
            AnyPublication::Post(_) => PublicationKind::Post,
            AnyPublication::PostTag => PublicationKind::PostTag,
            AnyPublication::Moderation => PublicationKind::Moderation,
            AnyPublication::UserEvent => PublicationKind::UserEvent,
            AnyPublication::StickerPack => PublicationKind::StickerPack,
            AnyPublication::Sticker => PublicationKind::Sticker,
            AnyPublication::ModerationEvent => PublicationKind::ModerationEvent,
            AnyPublication::AdminEvent => PublicationKind::AdminEvent,
            AnyPublication::FandomEvent => PublicationKind::FandomEvent,
            AnyPublication::Quest => PublicationKind::Quest,
        }
    }
}

impl Sealed for AnyPublication {}
