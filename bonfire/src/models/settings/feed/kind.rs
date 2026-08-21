#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the kinds of publications that can be shown in the feed.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Kind {
    /// Show only posts from followed accounts
    Follows,
    /// Show posts from all accounts
    All,
    /// Show posts with high karma
    Best,
    /// Show posts with mid karma
    Good,
    /// Show posts with low karma
    Abyss,
    /// Show all posts including those from followed accounts
    #[default]
    AllWithFollows,
}
