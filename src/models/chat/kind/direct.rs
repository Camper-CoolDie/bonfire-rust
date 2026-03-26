use chrono::{DateTime, Utc};

use crate::models::Account;
use crate::models::chat::{Messageable, Tag};
use crate::sealed::Sealed;

/// Represents a direct message chat.
#[derive(Default, Clone, Debug)]
pub struct Direct {
    /// The unique identifier of the authenticated user
    pub my_id: u64,
    /// The other account in this direct message chat
    pub recipient: Account,
    /// The timestamp when the other account last read the messages in this chat, if available
    pub recipient_read_at: Option<DateTime<Utc>>,
}

impl Messageable for Direct {
    /// Returns the chat's tag as [`ChatTag::Direct`][Tag::Direct].
    fn tag(&self) -> Tag {
        Tag::Direct {
            my_id: self.my_id,
            recipient_id: self.recipient.id,
        }
    }
}

impl Sealed for Direct {}
