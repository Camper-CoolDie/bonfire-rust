mod kind;

pub use kind::Kind;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{Category, Language};

/// Represents the user's feed settings.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Feed {
    /// The selected languages for filtering feed content, or empty if taken from the system
    pub languages: Vec<Language>,
    /// The selected categories for filtering feed content
    pub categories: Vec<Category>,
    /// Whether to show important posts in the feed
    pub show_important: bool,
    /// Whether to show closed posts in the feed
    pub show_closed: bool,
    /// The kinds of posts to show in the feed, or empty if no feed kind has been selected yet
    pub kinds: Vec<Kind>,
}
