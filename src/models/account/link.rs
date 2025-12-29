use serde::{Deserialize, Serialize};

/// Represents a link in an account's profile.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Link {
    /// The link's title
    pub title: String,
    /// The link's URI
    #[serde(rename = "url")]
    pub uri: String,
}
