#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, FandomRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CuratorAssigned {
    pub old_curator_id: Option<u64>,
    pub fandom: FandomRef,
    pub admin: AccountRef,
    pub reason: String,
}
