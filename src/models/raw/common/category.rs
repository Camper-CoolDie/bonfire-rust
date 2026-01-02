use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::Category;

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub(crate) enum RawCategory {
    #[serde(other)]
    Unknown = 101,
    Games = 1,
    Anime = 2,
    Movies = 5,
    Books = 8,
    Art = 15,
    Roleplay = 16,
    Other = 100,
}

impl From<RawCategory> for Category {
    fn from(value: RawCategory) -> Self {
        match value {
            RawCategory::Unknown => Category::Unknown,
            RawCategory::Games => Category::Games,
            RawCategory::Anime => Category::Anime,
            RawCategory::Movies => Category::Movies,
            RawCategory::Books => Category::Books,
            RawCategory::Art => Category::Art,
            RawCategory::Roleplay => Category::Roleplay,
            RawCategory::Other => Category::Other,
        }
    }
}

impl From<Category> for RawCategory {
    fn from(value: Category) -> Self {
        match value {
            Category::Unknown => RawCategory::Unknown,
            Category::Games => RawCategory::Games,
            Category::Anime => RawCategory::Anime,
            Category::Movies => RawCategory::Movies,
            Category::Books => RawCategory::Books,
            Category::Art => RawCategory::Art,
            Category::Roleplay => RawCategory::Roleplay,
            Category::Other => RawCategory::Other,
        }
    }
}
