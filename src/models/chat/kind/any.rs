use crate::models::chat::{Messageable, Tag};
use crate::models::{Direct, FandomRoot, FandomSub, Group};
use crate::sealed::Sealed;

/// Represents a union of all possible specific chat types.
///
/// This enum acts as a catch-all for various chat kinds when the specific type is not known
/// or needed, storing the specific data relevant to that type.
#[derive(Clone, Debug)]
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
