use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::chat::{Messageable, Tag};
use crate::models::{Account, Chat};
use crate::sealed::Sealed;

/// Represents a direct message chat.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Direct {
    /// The unique identifier of the authenticated user
    pub my_id: u64,
    /// The other account in this direct message chat
    pub recipient: Account,
    /// The timestamp when the other account last read the messages in this chat, if available
    pub recipient_read_at: Option<DateTime<Utc>>,
}
impl Direct {
    pub(crate) fn new(my_id: u64, recipient_id: u64) -> Self {
        Direct {
            my_id,
            recipient: Account::new(recipient_id),
            ..Default::default()
        }
    }
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

impl Chat<Direct> {
    #[must_use]
    pub fn new(my_id: u64, recipient_id: u64) -> Self {
        Self {
            spec: Direct::new(my_id, recipient_id),
            ..Default::default()
        }
    }
}
