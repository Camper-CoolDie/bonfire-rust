#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Reviewed {
    pub is_accepted: bool,
    pub fandom_id: u64,
    pub fandom_name: String,
    pub admin_name: String,
    pub note: String,
}
