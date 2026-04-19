use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::Category;

pub(crate) enum RawCategory {
    Games,
    Anime,
    Movies,
    Books,
    Art,
    Roleplay,
    Other,
    Unknown(i64),
}

impl Serialize for RawCategory {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let category = match self {
            RawCategory::Games => 1,
            RawCategory::Anime => 2,
            RawCategory::Movies => 5,
            RawCategory::Books => 8,
            RawCategory::Art => 15,
            RawCategory::Roleplay => 16,
            RawCategory::Other => 100,
            RawCategory::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(category)
    }
}

impl<'de> Deserialize<'de> for RawCategory {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawCategory::Games,
            2 => RawCategory::Anime,
            5 => RawCategory::Movies,
            8 => RawCategory::Books,
            15 => RawCategory::Art,
            16 => RawCategory::Roleplay,
            100 => RawCategory::Other,
            other => RawCategory::Unknown(other),
        })
    }
}

impl From<RawCategory> for Category {
    fn from(value: RawCategory) -> Self {
        match value {
            RawCategory::Games => Category::Games,
            RawCategory::Anime => Category::Anime,
            RawCategory::Movies => Category::Movies,
            RawCategory::Books => Category::Books,
            RawCategory::Art => Category::Art,
            RawCategory::Roleplay => Category::Roleplay,
            RawCategory::Other => Category::Other,
            RawCategory::Unknown(unknown) => Category::Unknown(unknown),
        }
    }
}

impl From<RawCategory> for Option<Category> {
    fn from(value: RawCategory) -> Self {
        match value {
            RawCategory::Unknown(0) => None,
            RawCategory::Games => Some(Category::Games),
            RawCategory::Anime => Some(Category::Anime),
            RawCategory::Movies => Some(Category::Movies),
            RawCategory::Books => Some(Category::Books),
            RawCategory::Art => Some(Category::Art),
            RawCategory::Roleplay => Some(Category::Roleplay),
            RawCategory::Other => Some(Category::Other),
            RawCategory::Unknown(category) => Some(Category::Unknown(category)),
        }
    }
}

impl From<Category> for RawCategory {
    fn from(value: Category) -> Self {
        match value {
            Category::Games => RawCategory::Games,
            Category::Anime => RawCategory::Anime,
            Category::Movies => RawCategory::Movies,
            Category::Books => RawCategory::Books,
            Category::Art => RawCategory::Art,
            Category::Roleplay => RawCategory::Roleplay,
            Category::Other => RawCategory::Other,
            Category::Unknown(unknown) => RawCategory::Unknown(unknown),
        }
    }
}
