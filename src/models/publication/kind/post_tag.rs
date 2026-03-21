use crate::models::ImageRef;
use crate::models::publication::{Kind, Publishable};
use crate::sealed::Sealed;

/// Represents a simple tag that can be attached to a post.
#[derive(Default, Clone, Debug)]
pub struct PostTag {
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
