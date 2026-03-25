mod content;
mod ref_content;
mod reference;

use std::ops::RangeInclusive;

pub use content::*;
pub use ref_content::RefContent;
pub use reference::Reference;

use crate::models::publication::{Kind, Publishable};
use crate::sealed::Sealed;

/// The allowed range for a comment's text length.
pub const COMMENT_TEXT_LENGTH_RANGE: RangeInclusive<usize> = 1..=4000;

/// Represents the specific data for a comment publication, containing text, and optionally a media
/// [`CommentContent`][Content] or a [`CommentRef`][Reference] to another publication.
#[derive(Default, Clone, Debug)]
pub struct Comment {
    /// The content associated with this comment
    pub content: Content,
    /// The text content of the comment, if any
    pub text: Option<String>,
    /// A reference to the publication this comment is replying to, if any
    pub reply_to: Option<Reference>,
    /// The name of the user this comment is directly answering to, if any. This is distinct from
    /// [`Comment::reply_to`], which points to a specific referenced comment
    pub answering_name: Option<String>,
    /// Indicates if the comment has been edited
    pub is_edited: bool,
    /// Indicates if the comment uses new Markdown formatting
    pub has_new_formatting: bool,
}

impl Publishable for Comment {
    /// Returns the publication kind as [`Kind::Comment`].
    fn kind(&self) -> Kind {
        Kind::Comment
    }
}

impl Sealed for Comment {}
