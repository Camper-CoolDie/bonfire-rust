use serde::Deserialize;

use crate::models::notification::ChatMessageCreated;
use crate::models::{ChatMessage, Publication};
use crate::requests::raw::{RawChatMessage, RawChatTag, RawPublication};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawMessageCreated {
    #[serde(rename = "unitChatMessage")]
    pub message: RawPublication<RawChatMessage>,
    #[serde(rename = "tag")]
    pub chat_tag: RawChatTag,
    #[serde(rename = "subscribed")]
    pub is_subscribed: bool,
}

impl TryFrom<RawMessageCreated> for ChatMessageCreated {
    type Error = Error;

    fn try_from(value: RawMessageCreated) -> Result<Self> {
        // Message has an empty chat tag
        let mut message = Publication::<ChatMessage>::try_from(value.message)?;
        message.content.chat_tag = value.chat_tag.try_into()?;

        Ok(Self {
            message,
            is_subscribed: value.is_subscribed,
        })
    }
}
