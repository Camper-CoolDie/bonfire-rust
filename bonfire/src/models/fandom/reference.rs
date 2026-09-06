#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Language;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Reference {
    pub id: u64,
    pub language: Option<Language>,
    pub name: String,
}
