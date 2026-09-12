#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Gender;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct FieldSet {
    pub admin_name: String,
    pub admin_gender: Gender,
    pub reason: String,
}
