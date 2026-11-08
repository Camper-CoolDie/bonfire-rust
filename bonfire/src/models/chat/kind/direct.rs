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
    pub ids: (u64, u64),
    /// The other account in this direct message chat
    pub recipient: Account,
    /// The timestamp when the other account last read the messages in this chat, if available
    pub recipient_read_at: Option<DateTime<Utc>>,
}
impl Direct {
    pub(crate) fn new(ids: (u64, u64)) -> Self {
        Direct {
            ids,
            ..Default::default()
        }
    }
}

impl Messageable for Direct {
    /// Returns the chat's tag as [`ChatTag::Direct`][Tag::Direct].
    fn tag(&self) -> Tag {
        Tag::Direct { ids: self.ids }
    }
}

impl Sealed for Direct {}

impl Chat<Direct> {
    #[must_use]
    pub fn new(ids: (u64, u64)) -> Self {
        Self {
            spec: Direct::new(ids),
            ..Default::default()
        }
    }
}
