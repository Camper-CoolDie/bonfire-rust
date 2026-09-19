#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, FandomRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RelayTurnMissed {
    pub id: u64,
    pub name: String,
    pub fandom: FandomRef,
    pub next_account: Option<AccountRef>,
}
