#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Gender;
use crate::models::account::EffectKind;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct EffectRemoved {
    pub id: u64,
    pub kind: EffectKind,
    pub admin_name: String,
    pub admin_gender: Gender,
    pub reason: String,
}
