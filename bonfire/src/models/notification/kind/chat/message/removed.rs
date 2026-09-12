#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Removed {
    pub id: u64,
}
