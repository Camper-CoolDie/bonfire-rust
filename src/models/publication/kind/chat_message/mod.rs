mod content;
mod ref_content;
mod reference;

use std::ops::RangeInclusive;

pub use content::*;
pub use ref_content::RefContent;
pub use reference::Reference;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::publication::{Kind, Publishable};
use crate::models::{Account, ChatTag, Fandom};
use crate::sealed::Sealed;

/// The allowed range for a chat message's text length.
pub const CHAT_MESSAGE_TEXT_LENGTH_RANGE: RangeInclusive<usize> = 1..=2000;

/// Represents the specific data for a chat message publication, containing text, and optionally a
/// media [`Content`][content::Content] or a [`ChatMessageRef`][Reference] to another publication.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ChatMessage {
    /// The content associated with this chat message
    pub content: Content,
    /// The fandom in which this chat message was posted, if applicable. Direct messages may not
    /// have an associated fandom
    pub fandom: Option<Fandom>,
    /// The account that authored this chat message, if applicable. System messages may not have an
    /// author
    pub author: Option<Account>,
    /// The tag identifying the chat this message belongs to
    pub chat_tag: ChatTag,
    /// The text content of the chat message, if any
    pub text: Option<String>,
    /// A reference to the publication this chat message is replying to, if any
    pub reply_to: Option<Reference>,
    /// The name of the user this chat message is directly answering to, if any. This is distinct
    /// from [`ChatMessage::reply_to`], which points to a specific referenced message
    pub answering_name: Option<String>,
    /// Indicates if the chat message has been edited
    pub is_edited: bool,
    /// Indicates if the chat message uses new Markdown formatting
    pub has_new_formatting: bool,
}

impl Publishable for ChatMessage {
    /// Returns the publication kind as [`Kind::ChatMessage`].
    fn to_kind(&self) -> Kind {
        Kind::ChatMessage
    }
}

impl Sealed for ChatMessage {}
