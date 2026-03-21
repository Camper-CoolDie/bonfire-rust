mod status;

use chrono::DateTime;
use serde::Deserialize;
use serde::de::Error as _;
pub(crate) use status::RawStatus;

use crate::models::Fandom;
use crate::requests::raw::{RawCategory, RawImageRef, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawFandom {
    pub id: u64,
    #[serde(rename = "languageId")]
    pub language: RawLanguage,
    #[serde(rename = "image")]
    pub icon: RawImageRef,
    #[serde(rename = "imageTitle")]
    pub background: RawImageRef,
    #[serde(rename = "imageTitleGif")]
    pub background_gif: RawImageRef,
    #[serde(rename = "closed")]
    pub is_closed: bool,
    #[serde(rename = "karmaCof")]
    pub karma_coef: f64,
    #[serde(rename = "creatorId")]
    pub suggester_id: u64,
    #[serde(rename = "dateCreate")]
    pub suggested_at: i64,
    #[serde(rename = "subscribesCount")]
    pub subscribers_count: u64,
    pub status: RawStatus,
    pub category: RawCategory,
}

impl TryFrom<RawFandom> for Fandom {
    type Error = Error;

    fn try_from(value: RawFandom) -> Result<Self> {
        Ok(Self {
            id: value.id,
            language: value.language.try_into()?,
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
            status: value.status.try_into()?,
            category: value.category.into(),
        })
    }
}
