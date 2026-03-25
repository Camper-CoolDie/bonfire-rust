use crate::models::chat::{Messageable, Tag};
use crate::models::{ImageRef, Language};
use crate::sealed::Sealed;

/// Represents a fandom root chat.
#[derive(Default, Clone, Debug)]
pub struct FandomRoot {
    /// The unique identifier of the fandom
    pub fandom_id: u64,
    /// The language of this fandom chat
    pub language: Language,
    /// The name of the fandom
    pub name: String,
    /// The icon image of the fandom
    pub icon: ImageRef,
    /// Indicates if the authenticated user is subscribed to this chat
    pub is_subscribed: bool,
    /// The number of subscribers to this chat
    pub subscribers_count: u64,
}

impl Messageable for FandomRoot {
    /// Returns the chat's tag as [`ChatTag::FandomRoot`][Tag::FandomRoot].
    fn tag(&self) -> Tag {
        Tag::FandomRoot {
            fandom_id: self.fandom_id,
            language: self.language.clone(),
        }
    }
}

impl Sealed for FandomRoot {}
