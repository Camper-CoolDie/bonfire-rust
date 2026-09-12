use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Punished {
    pub banned_until: Option<DateTime<Utc>>,
    pub reason: String,
}
