#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{Gender, Language};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct BlockRejected {
    pub moderation_id: u64,
    pub fandom_id: u64,
    pub fandom_language: Language,
    pub admin_name: String,
    pub admin_gender: Gender,
    pub reason: String,
}
