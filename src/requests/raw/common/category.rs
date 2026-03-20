use crate::models::Category;

pub(crate) enum RawCategory {
    Unknown(i64),
    Games,
    Anime,
    Movies,
    Books,
    Art,
    Roleplay,
    Other,
}

impl From<i64> for RawCategory {
    fn from(value: i64) -> Self {
        match value {
            1 => RawCategory::Games,
            2 => RawCategory::Anime,
            5 => RawCategory::Movies,
            8 => RawCategory::Books,
            15 => RawCategory::Art,
            16 => RawCategory::Roleplay,
            100 => RawCategory::Other,
            other => RawCategory::Unknown(other),
        }
    }
}

impl From<RawCategory> for Category {
    fn from(value: RawCategory) -> Self {
        match value {
            RawCategory::Unknown(category) => Category::Unknown(category),
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

impl From<RawCategory> for Option<Category> {
    fn from(value: RawCategory) -> Self {
        match value {
            RawCategory::Unknown(0) => None,
            RawCategory::Unknown(category) => Some(Category::Unknown(category)),
            RawCategory::Games => Some(Category::Games),
            RawCategory::Anime => Some(Category::Anime),
            RawCategory::Movies => Some(Category::Movies),
            RawCategory::Books => Some(Category::Books),
            RawCategory::Art => Some(Category::Art),
            RawCategory::Roleplay => Some(Category::Roleplay),
            RawCategory::Other => Some(Category::Other),
        }
    }
}
