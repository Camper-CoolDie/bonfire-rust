#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a category that a fandom can belong to.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Category {
    /// The fandom represents a game
    #[default]
    Games,
    /// The fandom represents an anime
    Anime,
    /// The fandom represents a movie
    Movies,
    /// The fandom represents a book
    Books,
    /// The fandom represents a specific type of art
    Art,
    /// The fandom represents a roleplay game
    Roleplay,
    /// The fandom has a non-standard category
    Other,
    /// The fandom has an unknown category
    Unknown(i64),
}
