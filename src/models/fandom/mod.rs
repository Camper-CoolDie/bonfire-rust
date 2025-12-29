use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::{Category, ImageRef, Language};

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
