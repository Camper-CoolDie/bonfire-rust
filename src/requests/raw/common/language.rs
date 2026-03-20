use serde::de::Error as _;

use crate::models::Language;
use crate::{Error, Result};

pub(crate) enum RawLanguage {
    Unknown(i64),
    English,
    Russian,
    Portuguese,
    Ukrainian,
    German,
    Italian,
    Polish,
    French,
}

impl From<i64> for RawLanguage {
    fn from(value: i64) -> Self {
        match value {
            1 => RawLanguage::English,
            2 => RawLanguage::Russian,
            3 => RawLanguage::Portuguese,
            4 => RawLanguage::Ukrainian,
            5 => RawLanguage::German,
            6 => RawLanguage::Italian,
            7 => RawLanguage::Polish,
            8 => RawLanguage::French,
            other => RawLanguage::Unknown(other),
        }
    }
}

impl TryFrom<RawLanguage> for Language {
    type Error = Error;

    fn try_from(value: RawLanguage) -> Result<Self> {
        Ok(match value {
            RawLanguage::English => Language::English,
            RawLanguage::Russian => Language::Russian,
            RawLanguage::Portuguese => Language::Portuguese,
            RawLanguage::Ukrainian => Language::Ukrainian,
            RawLanguage::German => Language::German,
            RawLanguage::Italian => Language::Italian,
            RawLanguage::Polish => Language::Polish,
            RawLanguage::French => Language::French,
            RawLanguage::Unknown(unknown) => Err(serde_json::Error::custom(format!(
                "invalid value: {}, expected one of: {}",
                unknown,
                (1..=8)
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )))?,
        })
    }
}

impl TryFrom<RawLanguage> for Option<Language> {
    type Error = Error;

    fn try_from(value: RawLanguage) -> Result<Self> {
        Ok(match value {
            // 0 is rarely used and means an unspecified language, rather than a multilingual fandom
            // which is returned as -1
            RawLanguage::Unknown(-1 | 0) => None,
            RawLanguage::English => Some(Language::English),
            RawLanguage::Russian => Some(Language::Russian),
            RawLanguage::Portuguese => Some(Language::Portuguese),
            RawLanguage::Ukrainian => Some(Language::Ukrainian),
            RawLanguage::German => Some(Language::German),
            RawLanguage::Italian => Some(Language::Italian),
            RawLanguage::Polish => Some(Language::Polish),
            RawLanguage::French => Some(Language::French),
            RawLanguage::Unknown(unknown) => Err(serde_json::Error::custom(format!(
                "invalid value: {}, expected one of: {}",
                unknown,
                (-1..=8)
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )))?,
        })
    }
}

impl From<Language> for i64 {
    fn from(value: Language) -> Self {
        match value {
            Language::English => 1,
            Language::Russian => 2,
            Language::Portuguese => 3,
            Language::Ukrainian => 4,
            Language::German => 5,
            Language::Italian => 6,
            Language::Polish => 7,
            Language::French => 8,
        }
    }
}
