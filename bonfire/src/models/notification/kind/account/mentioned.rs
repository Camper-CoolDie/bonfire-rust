#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, ChatTag, PublicationRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AccountMentioned {
    pub account: AccountRef,
    pub publication: PublicationRef,
    pub chat_tag: Option<ChatTag>,
    pub text: String,
}
