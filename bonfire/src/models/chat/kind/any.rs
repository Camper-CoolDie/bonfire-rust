use futures::Stream;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::client::Request as _;
use crate::models::chat::{Messageable, Tag};
use crate::models::{Chat, Direct, FandomRoot, FandomSub, Group};
use crate::requests::chat::{GetChatRequest, ListChatsRequest};
use crate::sealed::Sealed;
use crate::{Client, Result};

/// Represents a union of all possible specific chat types.
///
/// This enum acts as a catch-all for various chat kinds when the specific type is not known
/// or needed, storing the specific data relevant to that type.
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum AnyChat {
    /// A fandom root chat
    FandomRoot(FandomRoot),
    /// A fandom sub-chat
    FandomSub(FandomSub),
    /// A group chat
    Group(Group),
    /// A direct message chat
    Direct(Direct),
}

impl Messageable for AnyChat {
    fn tag(&self) -> Tag {
        match self {
            AnyChat::FandomRoot(chat) => chat.tag(),
            AnyChat::FandomSub(chat) => chat.tag(),
            AnyChat::Group(chat) => chat.tag(),
            AnyChat::Direct(chat) => chat.tag(),
        }
    }
}

impl Sealed for AnyChat {}

impl Default for AnyChat {
    fn default() -> Self {
        Self::FandomRoot(FandomRoot::default())
    }
}

impl Chat<AnyChat> {
    #[must_use]
    pub fn new(tag: Tag) -> Self {
        Self {
            spec: match tag {
                Tag::FandomRoot {
                    fandom_id,
                    language,
                } => AnyChat::FandomRoot(FandomRoot::new(fandom_id, language)),
                Tag::FandomSub { id } => AnyChat::FandomSub(FandomSub::new(id)),
                Tag::Group { id } => AnyChat::Group(Group::new(id)),
                Tag::Direct {
                    my_id,
                    recipient_id,
                } => AnyChat::Direct(Direct::new(my_id, recipient_id)),
            },
            ..Default::default()
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
    pub async fn get_by_tag(client: &Client, tag: Tag) -> Result<Self> {
        GetChatRequest::new_by_tag(tag).send(client).await
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
    pub async fn get_by_message_id(client: &Client, id: u64) -> Result<Self> {
        GetChatRequest::new_by_message_id(id).send(client).await
    }

    /// Retrieves a [`Stream`] of chats for the currently authenticated user.
    ///
    /// The returned chats are sorted by recent messages. This method returns a [`Stream`] that
    /// yields individual [`Chat`] instances as they are retrieved. The stream handles pagination
    /// automatically, fetching new pages of results as needed. The `offset` parameter can be used
    /// to skip a number of chats from the beginning of the list. If an [`Error`][crate::Error]
    /// occurs during the retrieval of any page, the stream will yield that single error and then
    /// terminate.
    pub fn list(client: &Client, offset: usize) -> impl Stream<Item = Result<Self>> {
        ListChatsRequest::new(offset).stream(client)
    }
}
