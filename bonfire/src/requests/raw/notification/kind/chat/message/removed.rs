use serde::Deserialize;

use crate::models::notification::ChatMessageRemoved;

#[derive(Deserialize)]
pub(crate) struct RawRemoved {
    #[serde(rename = "J_UNIT_ID")]
    pub id: u64,
}

impl From<RawRemoved> for ChatMessageRemoved {
    fn from(value: RawRemoved) -> Self {
        Self { id: value.id }
    }
}
