use crate::models::Language;

/// Represents a unique identifier for different types of chats.
///
/// This enum allows distinguishing between various chat contexts within the Bonfire API.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tag {
    /// Identifies a main chat for a fandom
    FandomRoot {
        /// The unique identifier of the fandom
        fandom_id: u64,
        /// The language of this fandom chat
        language: Language,
    },
    /// Identifies a specific sub-chat within a fandom
    FandomSub {
        /// The unique identifier of the sub-chat
        id: u64,
    },
    /// Identifies a standalone group chat
    Group {
        /// The unique identifier of the group chat
        id: u64,
    },
    /// Identifies a direct message conversation between two users
    Direct {
        /// The unique identifier of the authenticated user
        my_id: u64,
        /// The unique identifier of the other user in the conversation
        recipient_id: u64,
    },
}

impl Default for Tag {
    fn default() -> Self {
        Self::FandomRoot {
            fandom_id: u64::default(),
            language: Language::default(),
        }
    }
}
