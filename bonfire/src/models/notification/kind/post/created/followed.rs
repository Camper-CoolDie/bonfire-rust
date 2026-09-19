#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::AccountRef;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FollowedPostCreated {
    pub id: u64,
    pub author: AccountRef,
}
