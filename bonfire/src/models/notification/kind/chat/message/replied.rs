#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{ChatMessage, Publication};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct MessageReplied {
    pub reply: Publication<ChatMessage>,
    pub is_subscribed: bool,
}
