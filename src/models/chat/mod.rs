mod kind;
mod tag;

use chrono::{DateTime, Utc};
pub use kind::*;
pub use tag::Tag;

use crate::models::{ChatMessage, Publication};
use crate::sealed::Sealed;

/// A trait for chat types that extend the core [`Chat`] struct.
///
/// This trait allows for adding type-specific fields beyond the generic [`Chat`] data and provides
/// a way to get its identifying tag. [`AnyChat`] serves as a catch-all for chat kinds with
/// unspecified specific types.
pub trait Messageable: Sealed {
    /// Returns the specific tag that identifies this type of chat.
    fn tag(&self) -> Tag;
}

/// Represents various types of chat entities and their associated data.
///
/// This module provides structures and traits for handling different chat contexts, such as
/// fandom chats, group chats, and direct messages.
#[derive(Default, Clone, Debug)]
pub struct Chat<T: Messageable = AnyChat> {
    /// The specific kind of chat, holding its unique data
    pub kind: T,
    /// The last message sent in this chat, if any
    pub last_message: Option<Publication<ChatMessage>>,
    /// The number of unread messages in this chat
    pub unread_count: u64,
    /// The timestamp of when this chat was last marked as read
    pub read_at: Option<DateTime<Utc>>,
}
