#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::notification::{Kind, Notifiable};
use crate::models::{ChatMessage, ChatTag, Publication};
use crate::sealed::Sealed;

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

impl Notifiable for Chat {
    fn kind(&self) -> Kind {
        match self {
            Chat::MessageCreated { .. } => Kind::ChatMessageCreated,
            Chat::MessageEdited { .. } => Kind::ChatMessageEdited,
            Chat::MessageRemoved { .. } => Kind::ChatMessageRemoved,
            Chat::MessageReplied { .. } => Kind::ChatMessageReplied,
            Chat::Read { .. } => Kind::ChatRead,
            Chat::Typing { .. } => Kind::ChatTyping,
        }
    }
}

impl Sealed for Chat {}
