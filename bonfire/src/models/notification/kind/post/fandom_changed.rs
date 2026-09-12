#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, FandomRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FandomChanged {
    pub post_id: u64,
    pub old_fandom: FandomRef,
    pub new_fandom: FandomRef,
    pub admin: AccountRef,
    pub reason: String,
}
