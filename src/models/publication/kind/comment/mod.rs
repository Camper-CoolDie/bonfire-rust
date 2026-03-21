mod content;
mod reference;

pub use content::Content;
pub use reference::Reference;

use crate::models::publication::{Kind, Publishable};
use crate::sealed::Sealed;

/// Represents the specific data for a comment publication, containing text, and optionally a media
/// [`CommentContent`][Content] or a [`CommentReference`][Reference] to another publication.
#[derive(Default, Clone, Debug)]
pub struct Comment {
    /// The content associated with this comment
    pub content: Content,
    /// The text content of the comment, if any
    pub text: Option<String>,
    /// A reference to the publication this comment is replying to, if any
    pub reference: Option<Reference>,
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
