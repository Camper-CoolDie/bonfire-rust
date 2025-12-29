use serde::{Deserialize, Serialize};

use crate::models::Publication;

/// Represents a post.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    /// The post's inner struct
    #[serde(flatten)]
    pub inner: Publication,
    // #[serde(rename = "J_PAGES")]
    // pub pages: Vec<Page>,
    // pub best_comment: Option<Comment>,
    /// The post's rubric identifier
    #[serde(
        serialize_with = "crate::models::serialize_i64_or_none",
        deserialize_with = "crate::models::deserialize_i64_or_none"
    )]
    pub rubric_id: Option<i64>,
    /// The post's rubric name
    #[serde(
        serialize_with = "crate::models::serialize_string_or_none",
        deserialize_with = "crate::models::deserialize_string_or_none"
    )]
    pub rubric_name: Option<String>,
    /// The post's rubric karma coefficient
    #[serde(
        rename = "rubricKarmaCof",
        serialize_with = "crate::models::serialize_level_or_none",
        deserialize_with = "crate::models::deserialize_level_or_none"
    )]
    pub rubric_karma_coef: Option<f32>,
    // #[serde(rename = "userActivity")]
    // pub relay_race: Option<RelayRace>,
}
