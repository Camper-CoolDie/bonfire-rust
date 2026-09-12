use serde::Deserialize;

use crate::models::notification::ChatMessageCreated;
use crate::requests::raw::{RawChatMessage, RawChatTag, RawPublication};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawCreated {
    #[serde(rename = "unitChatMessage")]
    pub message: RawPublication<RawChatMessage>,
    #[serde(rename = "tag")]
    pub chat_tag: RawChatTag,
    #[serde(rename = "subscribed")]
    pub is_subscribed: bool,
}

impl TryFrom<RawCreated> for ChatMessageCreated {
    type Error = Error;

    fn try_from(value: RawCreated) -> Result<Self> {
        Ok(Self {
            message: value.message.try_into()?,
            chat_tag: value.chat_tag.try_into()?,
            is_subscribed: value.is_subscribed,
        })
    }
}
