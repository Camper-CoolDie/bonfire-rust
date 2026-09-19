use serde::Deserialize;

use crate::models::notification::ChatMessageReplied;
use crate::models::{ChatMessage, Publication};
use crate::requests::raw::{RawChatMessage, RawChatTag, RawPublication};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawMessageReplied {
    #[serde(rename = "unitChatMessage")]
    pub reply: RawPublication<RawChatMessage>,
    #[serde(rename = "tag")]
    pub chat_tag: RawChatTag,
    #[serde(rename = "subscribed")]
    pub is_subscribed: bool,
}

impl TryFrom<RawMessageReplied> for ChatMessageReplied {
    type Error = Error;

    fn try_from(value: RawMessageReplied) -> Result<Self> {
        // Reply has an empty chat tag
        let mut reply = Publication::<ChatMessage>::try_from(value.reply)?;
        reply.content.chat_tag = value.chat_tag.try_into()?;

        Ok(Self {
            reply,
            is_subscribed: value.is_subscribed,
        })
    }
}
