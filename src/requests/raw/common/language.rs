use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::Language;
use crate::{Error, Result};

#[derive(Default)]
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
    Unknown(i64),
}

impl Serialize for RawLanguage {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.into())
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
            other => RawLanguage::Unknown(other),
        }
    }
}

impl From<u64> for RawLanguage {
    fn from(value: u64) -> Self {
        value.cast_signed().into()
    }
}

impl From<&RawLanguage> for i64 {
    fn from(value: &RawLanguage) -> Self {
        match value {
            RawLanguage::English => 1,
            RawLanguage::Russian => 2,
            RawLanguage::Portuguese => 3,
            RawLanguage::Ukrainian => 4,
            RawLanguage::German => 5,
            RawLanguage::Italian => 6,
            RawLanguage::Polish => 7,
            RawLanguage::French => 8,
            RawLanguage::Unknown(unknown) => *unknown,
        }
    }
}

impl From<&RawLanguage> for u64 {
    fn from(value: &RawLanguage) -> Self {
        i64::from(value).cast_unsigned()
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
            RawLanguage::Unknown(unknown) => return Err(Error::UnknownVariant(unknown)),
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
            RawLanguage::Unknown(unknown) => return Err(Error::UnknownVariant(unknown)),
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
