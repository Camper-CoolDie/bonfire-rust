#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents filter settings for fandom publications.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Filter {
    /// Whether to show posts in the filter
    pub posts: bool,
    /// When enabled, shows only important posts (excludes events, moderations, etc.)
    pub only_important: bool,
    /// Whether to show events in the filter
    pub events: bool,
    /// Whether to show moderations in the filter
    pub moderations: bool,
    /// Whether to show moderation blocks (publications that were blocked)
    pub blocks: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            posts: true,
            only_important: false,
            events: true,
            moderations: true,
            blocks: true,
        }
    }
}
