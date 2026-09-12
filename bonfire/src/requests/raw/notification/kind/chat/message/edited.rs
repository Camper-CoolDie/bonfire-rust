use serde::Deserialize;

use crate::models::notification::ChatMessageEdited;

#[derive(Deserialize)]
pub(crate) struct RawEdited {
    #[serde(rename = "J_UNIT_ID")]
    pub id: u64,
    #[serde(rename = "J_TEXT")]
    pub new_text: String,
}

impl From<RawEdited> for ChatMessageEdited {
    fn from(value: RawEdited) -> Self {
        Self {
            id: value.id,
            new_text: value.new_text,
        }
    }
}
