mod content;
mod ref_content;
mod reference;

use std::ops::RangeInclusive;

pub use content::Content;
pub use ref_content::RefContent;
pub use reference::Reference;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::publication::{Kind, Publishable};
use crate::models::{Account, Fandom};
use crate::sealed::Sealed;

/// Represents the specific data for a comment publication, containing text, and optionally a media
/// [`CommentContent`][Content] or a [`CommentRef`][Reference] to another publication.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Comment {
    /// The content associated with this comment
    pub content: Content,
    /// The fandom in which this comment was posted
    pub fandom: Fandom,
    /// The account that authored this comment
    pub author: Account,
    /// The unique identifier of the parent publication this comment belongs to
    pub parent_id: u64,
    /// The type of the parent publication this comment belongs to
    pub parent_kind: Kind,
    /// The total karma received by this publication (can be positive or negative)
    pub karma: f64,
    /// The karma you personally placed on this publication, if any
    pub my_karma: Option<f64>,
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
impl Comment {
    /// The allowed range for a comment's text length.
    pub const TEXT_LENGTH_RANGE: RangeInclusive<usize> = 1..=4000;
}

impl Publishable for Comment {
    /// Returns the publication kind as [`Kind::Comment`].
    fn kind(&self) -> Kind {
        Kind::Comment
    }
}

impl Sealed for Comment {}
