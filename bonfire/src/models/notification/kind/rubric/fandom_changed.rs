#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, FandomRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FandomChanged {
    pub id: u64,
    pub name: String,
    pub moderation_id: u64,
    pub admin: AccountRef,
    pub old_fandom: FandomRef,
    pub new_fandom: FandomRef,
    pub reason: String,
}
