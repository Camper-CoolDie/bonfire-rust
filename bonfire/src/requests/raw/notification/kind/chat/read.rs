use serde::Deserialize;

use crate::models::ChatTag;
use crate::requests::raw::RawChatTag;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawRead {
    #[serde(rename = "tag")]
    pub chat_tag: RawChatTag,
}

impl TryFrom<RawRead> for ChatTag {
    type Error = Error;

    fn try_from(value: RawRead) -> Result<Self> {
        value.chat_tag.try_into()
    }
}
