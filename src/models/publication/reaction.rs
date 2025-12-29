use serde::{Deserialize, Serialize};

/// Represents a reaction on a publication.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Reaction {
    /// The reaction's account ID
    #[serde(rename = "accountId")]
    pub from_account_id: String,
    /// The reaction's index
    #[serde(rename = "reactionIndex")]
    pub index: i64,
}
