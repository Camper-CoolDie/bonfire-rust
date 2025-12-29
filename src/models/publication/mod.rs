mod kind;
mod reaction;

use chrono::{DateTime, Utc};
pub use kind::*;
pub use reaction::Reaction;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::models::{Account, Category, Fandom};

pub(crate) fn serialize_importance<S: Serializer>(
    value: &bool,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_i64(match value {
        true => -1,
        false => 0,
    })
}

pub(crate) fn deserialize_importance<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<bool, D::Error> {
    let value = i64::deserialize(deserializer)?;
    Ok(matches!(value, -1))
}

/// Represents a publication status.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum PublicationStatus {
    /// The publication's status is unspecified
    #[default]
    Unspecified = 0,
    /// The publication is a draft
    Draft = 1,
    /// The publication is published
    Published = 2,
    /// The publication is blocked
    Blocked = 3,
    /// The publication is deep-blocked (cannot be revert unless you're a protoadmin)
    DeepBlocked = 4,
    /// The publication is waiting to be published
    Pending = 5,
}

/// Represents a publication.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Publication {
    /// A unique identifier of this publication. Should always be set to a valid value if
    /// constructing with `{ ... }`
    pub id: i64,
    /// The publication's fandom
    pub fandom: Fandom,
    /// The publication's author
    #[serde(rename = "creator")]
    pub author: Account,
    /// The publication's category
    pub category: Category,
    /// The date when this publication was created (published)
    #[serde(
        rename = "dateCreate",
        serialize_with = "crate::models::serialize_timestamp_millis",
        deserialize_with = "crate::models::deserialize_timestamp_millis"
    )]
    pub created_at: DateTime<Utc>,
    /// The publication's type
    #[serde(rename = "unitType")]
    pub kind: PublicationKind,
    /// The parent publication's ID (if any)
    #[serde(
        rename = "parentUnitId",
        serialize_with = "crate::models::serialize_i64_or_none",
        deserialize_with = "crate::models::deserialize_i64_or_none"
    )]
    pub parent_id: Option<i64>,
    /// The parent publication's type (if any)
    #[serde(
        rename = "parentUnitType",
        serialize_with = "PublicationKind::serialize_or_none",
        deserialize_with = "PublicationKind::deserialize_or_none"
    )]
    pub parent_kind: Option<PublicationKind>,
    /// The publication's karma amount
    #[serde(
        rename = "karmaCount",
        serialize_with = "crate::models::serialize_level",
        deserialize_with = "crate::models::deserialize_level"
    )]
    pub karma: f32,
    /// The amount of karma you've placed on this publication
    #[serde(
        serialize_with = "crate::models::serialize_level_or_none",
        deserialize_with = "crate::models::deserialize_level_or_none"
    )]
    pub my_karma: Option<f32>,
    /// The publication's status
    pub status: PublicationStatus,
    /// Will this publication appear in feed? (not to be confused with [Fandom::is_closed])
    #[serde(rename = "closed")]
    pub is_closed: bool,
    /// The number of comments on this publication
    #[serde(rename = "subUnitsCount")]
    pub comments_count: i64,
    /// Is this publication marked as important?
    #[serde(
        rename = "important",
        serialize_with = "serialize_importance",
        deserialize_with = "deserialize_importance"
    )]
    pub is_important: bool,
    /// Does this publication come from a blacklisted fandom or account?
    #[serde(rename = "blacklisted")]
    pub is_blacklisted: bool,
    /// Is this publication marked as Not Safe For Work?
    #[serde(rename = "nsfw")]
    pub is_nsfw: bool,
    /// The publication's hotness
    pub hotness: f32,
    // TODO: tag_1, tag_2, tag_s_1, jsonDB.reactions, etc.
}
