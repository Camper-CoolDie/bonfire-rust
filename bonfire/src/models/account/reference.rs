#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Gender;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AccountRef {
    pub id: u64,
    pub name: String,
    pub gender: Gender,
}
