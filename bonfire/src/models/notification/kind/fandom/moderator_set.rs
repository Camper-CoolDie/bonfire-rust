#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::FandomRef;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ModeratorSet {
    pub fandom: FandomRef,
    pub reason: String,
}
