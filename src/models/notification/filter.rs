#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents notification filter settings.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Filter {
    /// Whether to show comment notifications
    pub comments: bool,
    /// Whether to show comment answer (reply) notifications
    pub answers: bool,
    /// Whether to show rate notifications
    pub rates: bool,
    /// Whether to show follow notifications
    pub follows: bool,
    /// Whether to show important post notifications
    pub important_posts: bool,
    /// Whether to show post notifications from followed accounts
    pub followed_posts: bool,
    /// Whether to show achievement notifications
    pub achievements: bool,
    /// Whether to show other types of notifications
    pub other: bool,
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            comments: true,
            answers: true,
            rates: true,
            follows: true,
            important_posts: true,
            followed_posts: true,
            achievements: true,
            other: true,
        }
    }
}
