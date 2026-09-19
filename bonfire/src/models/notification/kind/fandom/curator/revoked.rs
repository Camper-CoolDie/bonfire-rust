#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, FandomRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CuratorRevoked {
    pub fandom: FandomRef,
    pub admin: AccountRef,
    pub reason: String,
}
