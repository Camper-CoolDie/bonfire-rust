#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Account;

/// Identifiers of accounts that inherently possess the [`AccessLevel::Protoadmin`] privilege.
///
/// This list may change in the future, but currently includes only the main developer.
pub const PROTOADMIN_IDS: [u64; 1] = [1];

/// Represents the various access levels an account can hold within the platform.
///
/// These levels are hierarchical, meaning higher levels implicitly include privileges of lower
/// levels. The `Default` level is [`AccessLevel::User`].
#[derive(Default, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AccessLevel {
    /// Standard user level, the default for all accounts
    #[default]
    User,
    /// A trusted user level.
    ///
    /// Corresponds to level 3
    Trusted,
    /// An experienced user level.
    ///
    /// Corresponds to level 4.5 and requires 150 karma
    Experienced,
    /// A curator level.
    ///
    /// Corresponds to level 6 and requires 400 karma
    Curator,
    /// A moderator level.
    ///
    /// Corresponds to level 7 and requires 500 karma
    Moderator,
    /// An administrator level.
    ///
    /// Corresponds to level 8.5 and requires 700 karma
    Admin,
    /// A superadministrator level.
    ///
    /// Corresponds to level 10 and requires 1000 karma
    Superadmin,
    /// An expert user level.
    ///
    /// Corresponds to level 12 and requires 1300 karma
    Expert,
    /// Protoadministrator level, reserved for accounts explicitly listed in [`PROTOADMIN_IDS`]
    Protoadmin,
}

impl Account {
    /// Determines the [`AccessLevel`] of the account based on its current level and karma.
    ///
    /// Access levels are determined by the account's total `level` and the `karma30` (karma earned
    /// in the last 30 days). This means an account's access level can dynamically change
    /// (potentially decrease) based on recent activity, except for accounts listed in
    /// [`PROTOADMIN_IDS`], which always retain [`AccessLevel::Protoadmin`] privileges.
    ///
    /// This method relies on [`Account::id`], [`Account::level`] and [`Account::karma30`] fields
    /// being set.
    #[must_use]
    pub fn access_level(&self) -> AccessLevel {
        if PROTOADMIN_IDS.contains(&self.id) {
            AccessLevel::Protoadmin
        } else if self.level >= 12.0 && self.karma30 >= 1300.0 {
            AccessLevel::Expert
        } else if self.level >= 10.0 && self.karma30 >= 1000.0 {
            AccessLevel::Superadmin
        } else if self.level >= 8.5 && self.karma30 >= 700.0 {
            AccessLevel::Admin
        } else if self.level >= 7.0 && self.karma30 >= 500.0 {
            AccessLevel::Moderator
        } else if self.level >= 6.0 && self.karma30 >= 400.0 {
            AccessLevel::Curator
        } else if self.level >= 4.5 && self.karma30 >= 150.0 {
            AccessLevel::Experienced
        } else if self.level >= 3.0 {
            AccessLevel::Trusted
        } else {
            AccessLevel::User
        }
    }
}
