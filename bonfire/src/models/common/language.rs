#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a supported language.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Language {
    /// The English language
    #[default]
    #[cfg_attr(feature = "serde", serde(rename = "en"))]
    English,
    /// The Russian language
    #[cfg_attr(feature = "serde", serde(rename = "ru"))]
    Russian,
    /// The Portuguese language
    #[cfg_attr(feature = "serde", serde(rename = "pt"))]
    Portuguese,
    /// The Ukrainian language
    #[cfg_attr(feature = "serde", serde(rename = "uk"))]
    Ukrainian,
    /// The German language
    #[cfg_attr(feature = "serde", serde(rename = "de"))]
    German,
    /// The Italian language
    #[cfg_attr(feature = "serde", serde(rename = "it"))]
    Italian,
    /// The Polish language
    #[cfg_attr(feature = "serde", serde(rename = "pl"))]
    Polish,
    /// The French language
    #[cfg_attr(feature = "serde", serde(rename = "fr"))]
    French,
}
