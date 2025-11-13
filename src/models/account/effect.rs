use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::result::Result as StdResult;

/// Represents an account effect.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Effect {
    /// A unique identifier of this effect. Should always be set to a valid value if constructing
    /// with `{ ... }`
    pub id: i64,
    /// An account identifier to whom this effect is applied
    pub account_id: i64,
    /// The date when this effect was applied
    #[serde(
        rename = "dateCreate",
        serialize_with = "crate::models::serialize_timestamp_millis",
        deserialize_with = "crate::models::deserialize_timestamp_millis"
    )]
    pub applied_at: DateTime<Utc>,
    /// The effect's end date
    #[serde(
        rename = "dateEnd",
        serialize_with = "crate::models::serialize_timestamp_millis",
        deserialize_with = "crate::models::deserialize_timestamp_millis"
    )]
    pub ends_at: DateTime<Utc>,
    /// A reason for applying this effect. Empty if `is_system` is true
    #[serde(rename = "comment")]
    pub reason: String,
    /// The effect's type
    #[serde(rename = "effectIndex")]
    pub kind: EffectKind,
    /// Was this effect applied by the system?
    #[serde(
        rename = "tag",
        serialize_with = "Effect::serialize_tag",
        deserialize_with = "Effect::deserialize_tag"
    )]
    pub is_system: bool,
    /// A preselected reason for applying this effect. Useful only if `is_system` is true,
    /// otherwise use `reason`
    #[serde(rename = "commentTag")]
    pub reason_kind: EffectReasonKind,
    /// An account name who applied this effect. Empty if `is_system` is true
    pub from_account_name: String,
}
impl Effect {
    fn serialize_tag<S: Serializer>(value: &bool, serializer: S) -> StdResult<S::Ok, S::Error> {
        serializer.serialize_i64(*value as i64)
    }

    fn deserialize_tag<'de, D: Deserializer<'de>>(deserializer: D) -> StdResult<bool, D::Error> {
        Ok(i64::deserialize(deserializer)? == 1)
    }
}

/// Represents an effect type.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum EffectKind {
    /// The user can't place negative rates
    #[default]
    Hater = 1,
    /// The user's avatar is replaced with a pig
    Pig = 2,
    /// The user can't block publications
    Watchman = 3,
    /// A goose is running across the user's screen which you can't get rid of
    Goose = 4,
    /// It is constantly snowing for the user
    EternalWinter = 5,
    /// The user can't perform admin actions
    Punished = 6,
    /// The user has the privilege to translate the application regardless of their level and karma
    Translator = 7,
    /// The user can't mention others by using "@"
    MentionLock = 8,
}

/// Represents a preselected reason kind of an effect.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum EffectReasonKind {
    /// The reason is not preselected
    #[default]
    None = 0,
    /// Punished for inappropriate behavior towards the gods
    Gods = 1,
    /// Punished for unreasonable blocks
    RejectedBlocks = 2,
    /// Punished for blocking too many publications
    TooManyBlocks = 3,
    /// Punished for swearing in the service
    Swearing = 4,
    /// Punished for placing negative rates to every publication they saw
    Hater = 5,
    /// Punished for being uncultured
    Uncultured = 6,
}
