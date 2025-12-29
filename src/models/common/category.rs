use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents a category.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum Category {
    /// The fandom has an unknown or unspecified category
    #[default]
    #[serde(other)]
    Unknown = 101,
    /// The fandom represents a game
    Games = 1,
    /// The fandom represents an anime
    Anime = 2,
    /// The fandom represents a movie
    Movies = 5,
    /// The fandom represents a book
    Books = 8,
    /// The fandom represents a specific type of art
    Art = 15,
    /// The fandom represents a roleplay game
    Roleplay = 16,
    /// The fandom has a non-standart category
    Other = 100,
}
