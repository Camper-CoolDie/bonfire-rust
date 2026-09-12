#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Account;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AdminActionRejected {
    // TODO: admin actions
    // pub action: AdminAction,
    pub rejected_by: Account,
    pub created_by: Account,
    pub reason: String,
}
