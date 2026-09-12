#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, Language};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Transferred {
    pub id: u64,
    pub name: String,
    pub fandom_id: u64,
    pub fandom_language: Language,
    pub new_owner_id: u64,
    pub new_owner_name: String,
    pub moderation_id: u64,
    pub moderator: AccountRef,
    pub reason: String,
}
