use serde::{Deserialize, Serialize};

use crate::models::ImageRef;

/// Represents a badge in an account's profile.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Badge {
    /// The badge's index
    #[serde(rename = "id")]
    pub index: i64,
    /// The badge's image
    #[serde(rename = "mi")]
    pub image: ImageRef,
}
