#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Gender;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VisibilityChanged {
    pub moderation_id: u64,
    pub moderator_name: String,
    pub moderator_gender: Gender,
    pub reason: String,
}
