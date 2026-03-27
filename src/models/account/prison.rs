use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Account;

/// Represents an account that is currently banned.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PrisonEntry {
    /// The account that is banned
    pub account: Account,
    /// The date when this account's ban is scheduled to end
    pub banned_until: DateTime<Utc>,
}
