use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents a language.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum Language {
    /// English language
    #[default]
    English = 1,
    /// Russian language
    Russian = 2,
    /// Portuguese language
    Portuguese = 3,
    /// Ukrainian language
    Ukrainian = 4,
    /// Deutsch language
    Deutsch = 5,
    /// Italian language
    Italian = 6,
    /// Polish language
    Polish = 7,
    /// French language
    French = 8,
}
impl Language {
    pub(crate) fn serialize_or_none<S: Serializer>(
        value: &Option<Language>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(match value {
            None => 0,
            Some(value) => value.clone() as i64,
        })
    }

    pub(crate) fn deserialize_or_none<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Language>, D::Error> {
        let value = i64::deserialize(deserializer)?;
        Ok(match value {
            0 => None,
            1 => Some(Language::English),
            2 => Some(Language::Russian),
            3 => Some(Language::Portuguese),
            4 => Some(Language::Ukrainian),
            5 => Some(Language::Deutsch),
            6 => Some(Language::Italian),
            7 => Some(Language::Polish),
            8 => Some(Language::French),
            value => Err(D::Error::custom(format!(
                "invalid value: {}, expected one of: {}",
                value,
                (0..9)
                    .map(|i| i.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )))?,
        })
    }
}
