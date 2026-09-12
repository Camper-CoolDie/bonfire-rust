use serde::Deserialize;

use crate::models::notification::ChatMessageReplied;
use crate::requests::raw::{RawChatMessage, RawChatTag, RawPublication};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawReplied {
    #[serde(rename = "unitChatMessage")]
    pub reply: RawPublication<RawChatMessage>,
    #[serde(rename = "tag")]
    pub chat_tag: RawChatTag,
    #[serde(rename = "subscribed")]
    pub is_subscribed: bool,
}

impl TryFrom<RawReplied> for ChatMessageReplied {
    type Error = Error;

    fn try_from(value: RawReplied) -> Result<Self> {
        Ok(Self {
            reply: value.reply.try_into()?,
            chat_tag: value.chat_tag.try_into()?,
            is_subscribed: value.is_subscribed,
        })
    }
}
