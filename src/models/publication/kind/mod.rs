mod any;
mod chat_message;
mod comment;
mod post;
mod post_tag;

pub use any::AnyPublication;
pub use chat_message::{
    ChatMessage, Content as ChatMessageContent, RefContent as ChatMessageRefContent,
    Reference as ChatMessageRef,
};
pub use comment::{
    Comment, Content as CommentContent, RefContent as CommentRefContent, Reference as CommentRef,
};
pub use post::Post;
pub use post_tag::PostTag;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the specific type of a publication.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Kind {
    /// The publication is a post
    #[default]
    Post,
    /// The publication is a post tag
    PostTag,
    /// The publication is a comment
    Comment,
    /// The publication is a chat message
    ChatMessage,
    /// The publication is a moderation action
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
    /// The publication has an unknown type
    Unknown(i64),
}
