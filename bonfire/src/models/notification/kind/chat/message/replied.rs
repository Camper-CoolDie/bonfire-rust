#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{ChatMessage, ChatTag, Publication};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Replied {
    pub reply: Publication<ChatMessage>,
    pub chat_tag: ChatTag,
    pub is_subscribed: bool,
}
