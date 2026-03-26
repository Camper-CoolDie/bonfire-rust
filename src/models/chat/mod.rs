mod kind;
mod tag;
mod typing;

use chrono::{DateTime, Utc};
use futures::Stream;
pub use kind::*;
pub use tag::Tag;
pub use typing::Handler as TypingHandler;

use crate::client::Request as _;
use crate::models::streams::auto_paginated_stream;
use crate::models::{ChatMessage, Publication};
use crate::requests::chat::{GetChatRequest, GetChatsRequest};
use crate::sealed::Sealed;
use crate::{Client, Result};

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
impl Chat {
    /// Creates a new `Chat` instance from its tag.
    ///
    /// This is useful when you only need to reference a chat by its tag for sending associated
    /// requests. However, obtaining a fully populated `Chat` struct from methods like
    /// [`Chat::by_tag()`] or [`Chat::stream()`] is generally preferred.
    #[must_use]
    pub fn new(tag: Tag) -> Self {
        Self {
            kind: AnyChat::new(tag),
            ..Self::default()
        }
    }

    /// Retrieves a single chat by its tag.
    ///
    /// If successful and the chat is not already in the user's chat list, it will be added.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::UnsuccessfulResponse`][crate::Error::UnsuccessfulResponse] with the
    ///   status `500` if the chat is not a fandom root chat and does not exist.
    /// * Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if the chat is a
    ///   fandom root chat and does not exist.
    /// * Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if the chat is a group
    ///   and is private, or if the user has been removed from it.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn by_tag(client: &Client, tag: Tag) -> Result<Self> {
        GetChatRequest::new_by_tag(tag)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves a single chat by a message ID contained within it.
    ///
    /// If successful and the chat is not already in the user's chat list, it will be added.
    ///
    /// # Errors
    ///
    /// * Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if the
    ///   publication referenced by the given ID is not found.
    /// * Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if the publication
    ///   referenced by the given ID is not a chat message, or if the chat is a group and is
    ///   private, or if the user has been removed from it.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn by_message_id(client: &Client, id: u64) -> Result<Self> {
        GetChatRequest::new_by_message_id(id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves a [`Stream`] of chats for the currently authenticated user.
    ///
    /// The returned chats are sorted by recent messages. This method returns a [`Stream`] that
    /// yields individual [`Chat`] instances as they are retrieved. The stream handles pagination
    /// automatically, fetching new pages of results as needed. The `offset` parameter can be used
    /// to skip a number of chats from the beginning of the list. If an [`Error`][crate::Error]
    /// occurs during the retrieval of any page, the stream will yield that single error and then
    /// terminate.
    pub fn stream(client: &Client, offset: usize) -> impl Stream<Item = Result<Self>> + '_ {
        auto_paginated_stream(
            move |offset| async move {
                GetChatsRequest::new(offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            GetChatsRequest::PAGE_SIZE,
        )
    }
}
