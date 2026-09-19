#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::FandomRef;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct ImportantPostCreated {
    pub id: u64,
    pub fandom: FandomRef,
    pub importance_moderator_id: u64,
    pub importance_reason: String,
}
