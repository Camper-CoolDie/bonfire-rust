use crate::models::publication::{Kind, Publishable};
use crate::models::{Account, Fandom, ImageRef};
use crate::sealed::Sealed;

/// Represents the specific data for a simple tag that can be attached to a post.
#[derive(Default, Clone, Debug)]
pub struct PostTag {
    /// The fandom this tag belongs to
    pub fandom: Fandom,
    /// The account that created this tag
    pub creator: Account,
    /// The unique identifier of the category this tag belongs to, if any
    pub category_id: Option<u64>,
    /// The name of the tag
    pub name: String,
    /// The icon image for this tag, if set
    pub icon: Option<ImageRef>,
}

impl Publishable for PostTag {
    /// Returns the publication kind as [`Kind::PostTag`].
    fn kind(&self) -> Kind {
        Kind::PostTag
    }
}

impl Sealed for PostTag {}
