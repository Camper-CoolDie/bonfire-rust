mod status;

use chrono::DateTime;
use serde::Deserialize;
use serde::de::Error as _;
pub(crate) use status::RawFandomStatus;

use crate::models::Fandom;
use crate::requests::raw::{RawCategory, RawImageRef, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawFandom {
    pub id: u64,
    #[serde(rename = "languageId")]
    language: i64,
    #[serde(rename = "image")]
    icon: RawImageRef,
    #[serde(rename = "imageTitle")]
    background: RawImageRef,
    #[serde(rename = "imageTitleGif")]
    background_gif: RawImageRef,
    #[serde(rename = "closed")]
    is_closed: bool,
    #[serde(rename = "karmaCof")]
    karma_coef: f64,
    #[serde(rename = "creatorId")]
    suggester_id: u64,
    #[serde(rename = "dateCreate")]
    suggested_at: i64,
    #[serde(rename = "subscribesCount")]
    subscribers_count: u64,
    status: i64,
    category: i64,
}

impl TryFrom<RawFandom> for Fandom {
    type Error = Error;

    fn try_from(value: RawFandom) -> Result<Self> {
        Ok(Self {
            id: value.id,
            language: RawLanguage::from(value.language).try_into()?,
            icon: match value.icon.id {
                0 => None,
                _ => Some(value.icon.into()),
            },
            background: match value.background.id {
                0 => None,
                _ => Some(value.background.into()),
            },
            background_gif: match value.background_gif.id {
                0 => None,
                _ => Some(value.background_gif.into()),
            },
            is_closed: value.is_closed,
            karma_coef: value.karma_coef / 100.0,
            suggester_id: match value.suggester_id {
                0 => None,
                id => Some(id),
            },
            suggested_at: match value.suggested_at {
                0 => None,
                timestamp => Some(
                    DateTime::from_timestamp_millis(value.suggested_at).ok_or_else(|| {
                        serde_json::Error::custom(format!("timestamp {timestamp} is out of range"))
                    })?,
                ),
            },
            subscribers_count: value.subscribers_count,
            status: RawFandomStatus::from(value.status).try_into()?,
            category: RawCategory::from(value.category).into(),
        })
    }
}
