mod content;
mod ref_content;
mod reference;

use std::ops::RangeInclusive;

pub use content::*;
pub use ref_content::RefContent;
pub use reference::Reference;

use crate::models::publication::{Kind, Publishable};
use crate::sealed::Sealed;

/// The allowed range for a chat message's text length.
pub const CHAT_MESSAGE_TEXT_LENGTH_RANGE: RangeInclusive<usize> = 1..=2000;

/// Represents the specific data for a chat message publication, containing text, and optionally a
/// media [`Content`][content::Content] or a [`ChatMessageRef`][Reference] to another publication.
#[derive(Default, Clone, Debug)]
pub struct ChatMessage {
    /// The content associated with this chat message
    pub content: Content,
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
    fn kind(&self) -> Kind {
        Kind::ChatMessage
    }
}

impl Sealed for ChatMessage {}
