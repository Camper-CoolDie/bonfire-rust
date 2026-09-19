#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::FandomRef;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct RelayPostCreated {
    pub post_id: u64,
    pub fandom: FandomRef,
    pub relay_id: u64,
    pub relay_name: String,
}
