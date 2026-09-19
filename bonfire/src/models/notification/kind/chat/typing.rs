#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::ChatTag;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ChatTyping {
    pub account_id: u64,
    pub account_name: String,
    pub chat_tag: ChatTag,
}
