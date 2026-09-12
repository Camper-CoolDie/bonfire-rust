#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Edited {
    pub id: u64,
    pub new_text: String,
}
