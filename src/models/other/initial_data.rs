use chrono::{DateTime, Utc};

use crate::models::{Account, Settings};

/// Represents the initial data fetched from the server at startup.
#[derive(Clone, Debug)]
pub struct InitialData {
    /// The authenticated user's account information
    pub account: Account,
    /// The user's settings
    pub settings: Settings,
    /// A list of account IDs that have protoadmin privileges
    pub protoadmin_ids: Vec<u64>,
    /// The current server time
    pub server_time: DateTime<Utc>,
    /// Whether the user has any follows
    pub has_follows: bool,
}
