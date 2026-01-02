mod any;
mod post;

pub use any::AnyPublication;
pub use post::Post;

/// Represents a publication type.
#[derive(Default, Clone, Debug)]
pub enum PublicationKind {
    /// The publication has an unknown or unspecified type
    #[default]
    Unknown,
    /// The publication is a comment
    Comment,
    /// The publication is a chat message
    ChatMessage,
    /// The publication is a post
    Post,
    /// The publication is a post tag
    PostTag,
    /// The publication is a moderation
    Moderation,
    /// The publication is a user event
    UserEvent,
    /// The publication is a sticker pack
    StickerPack,
    /// The publication is a sticker
    Sticker,
    /// The publication is a moderation event
    ModerationEvent,
    /// The publication is an administration event
    AdminEvent,
    /// The publication is a fandom event
    FandomEvent,
    /// The publication is a quest
    Quest,
}
