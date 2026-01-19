use chrono::DateTime;
use serde::de::Error as _;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::models::fandom::FandomStatus;
use crate::models::{Fandom, Language};
use crate::requests::raw::{RawCategory, RawImageRef};
use crate::{Error, Result};

#[derive(Deserialize_repr)]
#[repr(i64)]
pub(crate) enum RawFandomStatus {
    Unspecified = 0,
    Suggested = 1,
    Accepted = 2,
}

impl From<RawFandomStatus> for FandomStatus {
    fn from(value: RawFandomStatus) -> Self {
        match value {
            RawFandomStatus::Unspecified => FandomStatus::Unspecified,
            RawFandomStatus::Suggested => FandomStatus::Suggested,
            RawFandomStatus::Accepted => FandomStatus::Accepted,
        }
    }
}

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
    karma_coef: i64,
    #[serde(rename = "creatorId")]
    suggester_id: u64,
    #[serde(rename = "dateCreate")]
    suggested_at: i64,
    #[serde(rename = "subscribesCount")]
    subscribers_count: u64,
    status: RawFandomStatus,
    category: RawCategory,
}

impl TryFrom<RawFandom> for Fandom {
    type Error = Error;

    fn try_from(value: RawFandom) -> Result<Self> {
        let language = match value.language {
            -1 => None,
            1 => Some(Language::English),
            2 => Some(Language::Russian),
            3 => Some(Language::Portuguese),
            4 => Some(Language::Ukrainian),
            5 => Some(Language::Deutsch),
            6 => Some(Language::Italian),
            7 => Some(Language::Polish),
            8 => Some(Language::French),
            language => Err(serde_json::Error::custom(format!(
                "invalid value: {}, expected one of: {}",
                language,
                [-1, 1, 2, 3, 4, 5, 6, 7, 8]
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )))?,
        };

        Ok(Self {
            id: value.id,
            language,
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
            karma_coef: value.karma_coef as f32 / 100.,
            suggester_id: match value.suggester_id {
                0 => None,
                id => Some(id),
            },
            suggested_at: match value.suggested_at {
                0 => None,
                timestamp => Some(
                    DateTime::from_timestamp_millis(value.suggested_at).ok_or_else(|| {
                        serde_json::Error::custom(format!(
                            "timestamp {} is out of range",
                            timestamp
                        ))
                    })?,
                ),
            },
            subscribers_count: value.subscribers_count,
            status: value.status.into(),
            category: value.category.into(),
        })
    }
}
