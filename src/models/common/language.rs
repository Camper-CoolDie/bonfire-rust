#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a supported language.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Language {
    /// The English language
    #[default]
    English,
    /// The Russian language
    Russian,
    /// The Portuguese language
    Portuguese,
    /// The Ukrainian language
    Ukrainian,
    /// The German language
    German,
    /// The Italian language
    Italian,
    /// The Polish language
    Polish,
    /// The French language
    French,
}
