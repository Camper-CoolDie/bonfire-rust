mod kind;
mod tag;

use chrono::{DateTime, Utc};
use futures::Stream;
pub use kind::*;
pub use tag::Tag;

use crate::client::Request as _;
use crate::models::streams::auto_paginated_stream;
use crate::models::{ChatMessage, Publication};
use crate::requests::chat::GetChatsRequest;
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
    /// Retrieves a [`Stream`] of chats for the currently authenticated user.
    ///
    /// The returned chats are sorted by recent messages. This method returns a [`Stream`] that
    /// yields individual [`Chat`] instances as they are retrieved. The stream handles pagination
    /// automatically, fetching new pages of results as needed. The `offset` parameter can be used
    /// to skip a number of chats from the beginning of the list. If an [`Error`][crate::Error]
    /// occurs during the retrieval of any page, the stream will yield that single error and then
    /// terminate.
    pub fn list(client: &Client, offset: usize) -> impl Stream<Item = Result<Self>> + '_ {
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
