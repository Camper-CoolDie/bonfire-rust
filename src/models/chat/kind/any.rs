#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::chat::{Messageable, Tag};
use crate::models::{Direct, FandomRoot, FandomSub, Group};
use crate::sealed::Sealed;

/// Represents a union of all possible specific chat types.
///
/// This enum acts as a catch-all for various chat kinds when the specific type is not known
/// or needed, storing the specific data relevant to that type.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
impl AnyChat {
    /// Creates a new `AnyChat` instance from a given [`Tag`].
    ///
    /// This method allows constructing an `AnyChat` enum variant based on the provided chat tag,
    /// providing a convenient way to instantiate a generic chat wrapper.
    #[must_use]
    pub fn new(tag: Tag) -> Self {
        match tag {
            Tag::FandomRoot {
                fandom_id,
                language,
            } => Self::FandomRoot(FandomRoot::new(fandom_id, language)),
            Tag::FandomSub { id } => Self::FandomSub(FandomSub::new(id)),
            Tag::Group { id } => Self::Group(Group::new(id)),
            Tag::Direct {
                my_id,
                recipient_id,
            } => Self::Direct(Direct::new(my_id, recipient_id)),
        }
    }
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
