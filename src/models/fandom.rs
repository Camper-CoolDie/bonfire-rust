use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::ImageRef;

/// Represents a fandom.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Fandom {
    /// A unique identifier of this fandom. Should always be set to a valid value if constructing
    /// with `{ ... }`
    pub id: i64,
    /// The fandom's language. Should be set in most cases if constructing with `{ ... }`. None if
    /// multilingual
    #[serde(
        rename = "languageId",
        serialize_with = "Language::serialize_or_none",
        deserialize_with = "Language::deserialize_or_none"
    )]
    pub language: Option<Language>,
    /// The fandom's avatar
    #[serde(
        rename = "image",
        serialize_with = "ImageRef::serialize_or_none",
        deserialize_with = "ImageRef::deserialize_or_none"
    )]
    pub icon: Option<ImageRef>,
    /// The fandom's background
    #[serde(
        rename = "imageTitle",
        serialize_with = "ImageRef::serialize_or_none",
        deserialize_with = "ImageRef::deserialize_or_none"
    )]
    pub background: Option<ImageRef>,
    /// The fandom's GIF background
    #[serde(
        rename = "imageTitleGif",
        serialize_with = "ImageRef::serialize_or_none",
        deserialize_with = "ImageRef::deserialize_or_none"
    )]
    pub background_gif: Option<ImageRef>,
    /// Will publications from this fandom appear in feed?
    #[serde(rename = "closed")]
    pub is_closed: bool,
    /// The fandom's karma coefficient
    #[serde(
        rename = "karmaCof",
        serialize_with = "crate::models::serialize_level",
        deserialize_with = "crate::models::deserialize_level"
    )]
    pub karma_coef: f32,
    /// An account identifier who suggested this fandom
    #[serde(
        rename = "creatorId",
        serialize_with = "crate::models::serialize_i64_or_none",
        deserialize_with = "crate::models::deserialize_i64_or_none"
    )]
    pub suggester_id: Option<i64>,
    /// The date when this fandom was suggested
    #[serde(
        rename = "dateCreate",
        serialize_with = "crate::models::serialize_timestamp_millis_or_none",
        deserialize_with = "crate::models::deserialize_timestamp_millis_or_none"
    )]
    pub suggested_at: Option<DateTime<Utc>>,
    /// The number of users who subscribed to this fandom
    #[serde(rename = "subscribesCount")]
    pub subscribers_count: i64,
    /// The fandom's status
    pub status: FandomStatus,
    /// The fandom's category
    pub category: Category,
}

/// Represents a language.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum Language {
    /// English language
    #[default]
    English = 1,
    /// Russian language
    Russian = 2,
    /// Portuguese language
    Portuguese = 3,
    /// Ukrainian language
    Ukrainian = 4,
    /// Deutsch language
    Deutsch = 5,
    /// Italian language
    Italian = 6,
    /// Polish language
    Polish = 7,
    /// French language
    French = 8,
}
impl Language {
    pub(crate) fn serialize_or_none<S: Serializer>(
        value: &Option<Language>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(match value {
            None => 0,
            Some(value) => value.clone() as i64,
        })
    }

    pub(crate) fn deserialize_or_none<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Language>, D::Error> {
        let value = i64::deserialize(deserializer)?;
        Ok(match value {
            0 => None,
            2 => Some(Language::Russian),
            3 => Some(Language::Portuguese),
            4 => Some(Language::Ukrainian),
            5 => Some(Language::Deutsch),
            6 => Some(Language::Italian),
            7 => Some(Language::Polish),
            8 => Some(Language::French),
            _ => Some(Language::English),
        })
    }
}

/// Represents a fandom status.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum FandomStatus {
    /// The fandom's status is unspecified
    #[default]
    Unspecified = 0,
    /// The fandom is waiting for approval
    Suggested = 1,
    /// The fandom was accepted
    Accepted = 2,
}

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
