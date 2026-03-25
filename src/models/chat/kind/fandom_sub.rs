use crate::models::ImageRef;
use crate::models::chat::{Messageable, Tag};
use crate::sealed::Sealed;

/// Represents a fandom sub-chat.
#[derive(Default, Clone, Debug)]
pub struct FandomSub {
    /// The unique identifier of the sub-chat
    pub id: u64,
    /// The name of the sub-chat
    pub name: String,
    /// The icon image of the sub-chat
    pub icon: ImageRef,
    /// The background image of the sub-chat, if set
    pub background: Option<ImageRef>,
    /// The introductory message or description of the sub-chat, if set
    pub intro: Option<String>,
}

impl Messageable for FandomSub {
    /// Returns the chat's tag as [`ChatTag::FandomSub`][Tag::FandomSub].
    fn tag(&self) -> Tag {
        Tag::FandomSub { id: self.id }
    }
}

impl Sealed for FandomSub {}
