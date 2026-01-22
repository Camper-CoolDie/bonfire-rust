use serde::Deserialize;

use crate::models::Reaction;

#[derive(Deserialize)]
pub(crate) struct RawReaction {
    #[serde(rename = "reactionIndex")]
    index: i64,
    #[serde(rename = "accountId")]
    from_account_id: u64,
}

impl From<RawReaction> for Reaction {
    fn from(value: RawReaction) -> Self {
        Self {
            index: value.index,
            from_account_id: value.from_account_id,
        }
    }
}
