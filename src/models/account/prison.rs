use chrono::{DateTime, Utc};

use crate::models::Account;

/// Represents an account that is currently banned.
#[derive(Default, Clone, Debug)]
pub struct PrisonEntry {
    /// The account that is banned
    pub account: Account,
    /// The date when this account's ban is scheduled to end
    pub banned_until: DateTime<Utc>,
}
