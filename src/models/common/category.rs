/// Represents a category of a fandom.
#[derive(Default, Clone, Debug)]
pub enum Category {
    /// The fandom has an unknown or unspecified category
    #[default]
    Unknown,
    /// The fandom represents a game
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
    /// The fandom has a non-standart category
    Other,
}
