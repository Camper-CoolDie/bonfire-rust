use serde::Deserialize;

use crate::models::notification::ChatTyping;
use crate::requests::raw::RawChatTag;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawChatTyping {
    pub account_id: u64,
    pub account_name: String,
    pub chat_tag: RawChatTag,
}

impl TryFrom<RawChatTyping> for ChatTyping {
    type Error = Error;

    fn try_from(value: RawChatTyping) -> Result<Self> {
        Ok(Self {
            account_id: value.account_id,
            account_name: value.account_name,
            chat_tag: value.chat_tag.try_into()?,
        })
    }
}
