#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Language;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct KarmaCoefChanged {
    pub id: u64,
    pub name: String,
    pub fandom_id: u64,
    pub fandom_language: Language,
    pub old_coef: f64,
    pub new_coef: f64,
}
