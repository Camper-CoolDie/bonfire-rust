use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::Language;

#[derive(Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub(crate) enum RawLanguage {
    English = 1,
    Russian = 2,
    Portuguese = 3,
    Ukrainian = 4,
    Deutsch = 5,
    Italian = 6,
    Polish = 7,
    French = 8,
}

impl From<RawLanguage> for Language {
    fn from(value: RawLanguage) -> Self {
        match value {
            RawLanguage::English => Language::English,
            RawLanguage::Russian => Language::Russian,
            RawLanguage::Portuguese => Language::Portuguese,
            RawLanguage::Ukrainian => Language::Ukrainian,
            RawLanguage::Deutsch => Language::Deutsch,
            RawLanguage::Italian => Language::Italian,
            RawLanguage::Polish => Language::Polish,
            RawLanguage::French => Language::French,
        }
    }
}

impl From<Language> for RawLanguage {
    fn from(value: Language) -> Self {
        match value {
            Language::English => RawLanguage::English,
            Language::Russian => RawLanguage::Russian,
            Language::Portuguese => RawLanguage::Portuguese,
            Language::Ukrainian => RawLanguage::Ukrainian,
            Language::Deutsch => RawLanguage::Deutsch,
            Language::Italian => RawLanguage::Italian,
            Language::Polish => RawLanguage::Polish,
            Language::French => RawLanguage::French,
        }
    }
}
