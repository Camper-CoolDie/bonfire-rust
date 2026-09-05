#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{ChatMessage, ChatTag, Publication};

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Chat {
    MessageCreated {
        message: Publication<ChatMessage>,
        chat_tag: ChatTag,
        is_subscribed: bool,
    },
    MessageEdited {
        id: u64,
        new_text: String,
    },
    MessageRemoved {
        id: u64,
    },
    MessageReplied {
        reply: Publication<ChatMessage>,
        chat_tag: ChatTag,
        is_subscribed: bool,
    },
    Read {
        chat_tag: ChatTag,
    },
    Typing {
        account_id: u64,
        account_name: String,
        chat_tag: ChatTag,
    },
}
