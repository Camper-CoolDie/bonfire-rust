#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Language;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FandomRef {
    pub id: u64,
    pub language: Language,
    pub name: String,
}
