pub mod kind;
mod tag;
mod typing;

use chrono::{DateTime, Utc};
use futures::Stream;
pub use kind::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub use tag::Tag;
pub use typing::Handler as TypingHandler;

use crate::models::{ChatMessage, Publication};
pub use crate::requests::chat::ListMessagesAnchor;
use crate::requests::chat::ListMessagesRequest;
use crate::sealed::Sealed;
use crate::{Client, Result};

/// A trait for chat types that extend the core [`Chat`] struct.
///
/// This trait allows for adding type-specific fields beyond the generic [`Chat`] data and provides
/// a way to get its identifying tag. [`AnyChat`] serves as a catch-all for chat kinds with
/// unspecified specific types.
pub trait Messageable: Default + Sealed {
    /// Returns the specific tag that identifies this type of chat.
    #[must_use]
    fn tag(&self) -> Tag;
}

/// Represents various types of chat entities and their associated data.
///
/// This module provides structures and traits for handling different chat contexts, such as
/// fandom chats, group chats, and direct messages.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Chat<T: Messageable = AnyChat> {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub spec: T,
    /// The last message sent in this chat, if any
    pub last_message: Option<Publication<ChatMessage>>,
    /// The number of unread messages in this chat
    pub unread_count: u64,
    /// The timestamp of when this chat was last marked as read
    pub read_at: Option<DateTime<Utc>>,
}

impl<T: Messageable> Chat<T> {
    pub fn list_messages<'a>(
        &'a self,
        client: &'a Client,
        newest_first: bool,
        anchor: &ListMessagesAnchor,
    ) -> impl Stream<Item = Result<Publication<ChatMessage>>> + 'a {
        ListMessagesRequest::new(self.tag(), newest_first, anchor).stream(client)
    }

    #[must_use]
    pub fn tag(&self) -> Tag {
        self.spec.tag()
    }
}
