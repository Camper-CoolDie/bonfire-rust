#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents filter settings for account publications.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Filter {
    /// Whether to show posts in the filter
    pub posts: bool,
    /// Whether to show comments in the filter
    pub comments: bool,
    /// Whether to show chat messages in the filter
    pub chat_messages: bool,
    /// Whether to show events in the filter
    pub events: bool,
    /// Whether to show sticker-specific events.
    ///
    /// Sticker events include when a sticker was added to the user's collection or when
    /// the user created a new sticker pack.
    pub sticker_events: bool,
    /// Whether to show moderations in the filter
    pub moderations: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            posts: true,
            comments: true,
            chat_messages: false,
            events: true,
            sticker_events: true,
            moderations: true,
        }
    }
}
