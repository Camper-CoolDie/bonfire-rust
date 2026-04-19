use std::result::Result as StdResult;

use serde::de::Error as _;
use serde::{Deserialize, Serialize};

use crate::models::Language;
use crate::{Error, Result};

#[derive(Default, Debug)]
pub(crate) enum RawLanguage {
    #[default]
    English,
    Russian,
    Portuguese,
    Ukrainian,
    German,
    Italian,
    Polish,
    French,
    UnknownInteger(i64),
    UnknownString(String),
}

impl Serialize for RawLanguage {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.try_into().map_err(serde::ser::Error::custom)?)
    }
}

impl<'de> Deserialize<'de> for RawLanguage {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(i64::deserialize(deserializer)?.into())
    }
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
            other => RawLanguage::UnknownInteger(other),
        }
    }
}

impl From<u64> for RawLanguage {
    #[expect(clippy::cast_possible_wrap)]
    fn from(value: u64) -> Self {
        (value as i64).into()
    }
}

impl From<String> for RawLanguage {
    fn from(value: String) -> Self {
        match value.as_str() {
            "en" => RawLanguage::English,
            "ru" => RawLanguage::Russian,
            "pt" => RawLanguage::Portuguese,
            "uk" => RawLanguage::Ukrainian,
            "de" => RawLanguage::German,
            "it" => RawLanguage::Italian,
            "pl" => RawLanguage::Polish,
            "fr" => RawLanguage::French,
            _ => RawLanguage::UnknownString(value),
        }
    }
}

impl TryFrom<&RawLanguage> for i64 {
    type Error = String;

    fn try_from(value: &RawLanguage) -> StdResult<Self, String> {
        Ok(match value {
            RawLanguage::English => 1,
            RawLanguage::Russian => 2,
            RawLanguage::Portuguese => 3,
            RawLanguage::Ukrainian => 4,
            RawLanguage::German => 5,
            RawLanguage::Italian => 6,
            RawLanguage::Polish => 7,
            RawLanguage::French => 8,
            RawLanguage::UnknownInteger(unknown) => *unknown,
            RawLanguage::UnknownString(unknown) => {
                return Err(format!(
                    "cannot convert unknown language string {unknown:?} to integer"
                ));
            }
        })
    }
}

impl TryFrom<&RawLanguage> for u64 {
    type Error = String;

    #[expect(clippy::cast_sign_loss)]
    fn try_from(value: &RawLanguage) -> StdResult<Self, String> {
        Ok(i64::try_from(value)? as u64)
    }
}

impl TryFrom<RawLanguage> for String {
    type Error = Error;

    fn try_from(value: RawLanguage) -> Result<Self> {
        Ok(match value {
            RawLanguage::English => "en".to_owned(),
            RawLanguage::Russian => "ru".to_owned(),
            RawLanguage::Portuguese => "pt".to_owned(),
            RawLanguage::Ukrainian => "uk".to_owned(),
            RawLanguage::German => "de".to_owned(),
            RawLanguage::Italian => "it".to_owned(),
            RawLanguage::Polish => "pl".to_owned(),
            RawLanguage::French => "fr".to_owned(),
            RawLanguage::UnknownString(unknown) => unknown,
            RawLanguage::UnknownInteger(unknown) => {
                return Err(Error::JsonError(serde_json::Error::custom(format!(
                    "cannot convert unknown language integer {unknown:?} to string"
                ))));
            }
        })
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
            RawLanguage::UnknownInteger(_) | RawLanguage::UnknownString(_) => {
                return Err(Error::UnknownVariant(Box::new(value)));
            }
        })
    }
}

impl TryFrom<RawLanguage> for Option<Language> {
    type Error = Error;

    fn try_from(value: RawLanguage) -> Result<Self> {
        Ok(match value {
            // 0 is rarely used and means an unspecified language, rather than a multilingual fandom
            // which is returned as -1
            RawLanguage::UnknownInteger(-1 | 0) => None,
            RawLanguage::UnknownString(ref language) if language.is_empty() => None,
            RawLanguage::English => Some(Language::English),
            RawLanguage::Russian => Some(Language::Russian),
            RawLanguage::Portuguese => Some(Language::Portuguese),
            RawLanguage::Ukrainian => Some(Language::Ukrainian),
            RawLanguage::German => Some(Language::German),
            RawLanguage::Italian => Some(Language::Italian),
            RawLanguage::Polish => Some(Language::Polish),
            RawLanguage::French => Some(Language::French),
            RawLanguage::UnknownInteger(_) | RawLanguage::UnknownString(_) => {
                return Err(Error::UnknownVariant(Box::new(value)));
            }
        })
    }
}

impl From<Language> for RawLanguage {
    fn from(value: Language) -> Self {
        match value {
            Language::English => RawLanguage::English,
            Language::Russian => RawLanguage::Russian,
            Language::Portuguese => RawLanguage::Portuguese,
            Language::Ukrainian => RawLanguage::Ukrainian,
            Language::German => RawLanguage::German,
            Language::Italian => RawLanguage::Italian,
            Language::Polish => RawLanguage::Polish,
            Language::French => RawLanguage::French,
        }
    }
}
