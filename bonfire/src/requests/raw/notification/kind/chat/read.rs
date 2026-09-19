use serde::Deserialize;

use crate::models::ChatTag;
use crate::requests::raw::RawChatTag;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawChatRead {
    #[serde(rename = "tag")]
    pub chat_tag: RawChatTag,
}

impl TryFrom<RawChatRead> for ChatTag {
    type Error = Error;

    fn try_from(value: RawChatRead) -> Result<Self> {
        value.chat_tag.try_into()
    }
}
