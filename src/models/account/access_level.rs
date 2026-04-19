#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::models::{Account, Config};

// Requirements for all AccessLevel variants (except User and Protoadmin) in the format
// (level, karma30). When changing a requirement, also update the associated variant's doc
const TRUSTED_REQUIREMENT: (f64, f64) = (3.0, 0.0);
const EXPERIENCED_REQUIREMENT: (f64, f64) = (4.5, 150.0);
const CURATOR_REQUIREMENT: (f64, f64) = (6.0, 400.0);
const MODERATOR_REQUIREMENT: (f64, f64) = (7.0, 500.0);
const ADMIN_REQUIREMENT: (f64, f64) = (8.5, 700.0);
const SUPERADMIN_REQUIREMENT: (f64, f64) = (10.0, 1000.0);
const EXPERT_REQUIREMENT: (f64, f64) = (12.0, 1300.0);

/// Represents the various access levels an account can hold within the platform.
///
/// These levels are hierarchical, meaning higher levels implicitly include privileges of lower
/// levels. The `Default` level is [`AccessLevel::User`].
///
/// Each access level (except for [`AccessLevel::User`] and [`AccessLevel::Protoadmin`]) has
/// specific [`level`][AccessLevel::requirement()] and [`karma30`][AccessLevel::requirement()]
/// requirements that an account must meet to attain that level.
#[derive(Default, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, EnumIter)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum AccessLevel {
    /// Standard user level, the default for all accounts
    #[default]
    User,
    /// A trusted user level.
    ///
    /// Corresponds to level 3.
    Trusted,
    /// An experienced user level.
    ///
    /// Corresponds to level 4.5 and requires 150 karma.
    Experienced,
    /// A curator level.
    ///
    /// Corresponds to level 6 and requires 400 karma.
    Curator,
    /// A moderator level.
    ///
    /// Corresponds to level 7 and requires 500 karma.
    Moderator,
    /// An administrator level.
    ///
    /// Corresponds to level 8.5 and requires 700 karma.
    Admin,
    /// A superadministrator level.
    ///
    /// Corresponds to level 10 and requires 1000 karma.
    Superadmin,
    /// An expert user level.
    ///
    /// Corresponds to level 12 and requires 1300 karma.
    Expert,
    /// Protoadministrator level, reserved for accounts explicitly listed in
    /// [`Config::protoadmin_ids`]
    Protoadmin,
}
impl AccessLevel {
    /// Returns the minimum `(level, karma30)` requirements for this access level.
    ///
    /// The first element of the tuple represents the minimum account `level` required, and the
    /// second element represents the minimum `karma30` (karma earned in the last 30 days) required.
    ///
    /// Returns `None` for [`AccessLevel::User`] and [`AccessLevel::Protoadmin`] as they don't have
    /// specific dynamic requirements.
    #[must_use]
    pub fn requirement(&self) -> Option<(f64, f64)> {
        match self {
            AccessLevel::Trusted => Some(TRUSTED_REQUIREMENT),
            AccessLevel::Experienced => Some(EXPERIENCED_REQUIREMENT),
            AccessLevel::Curator => Some(CURATOR_REQUIREMENT),
            AccessLevel::Moderator => Some(MODERATOR_REQUIREMENT),
            AccessLevel::Admin => Some(ADMIN_REQUIREMENT),
            AccessLevel::Superadmin => Some(SUPERADMIN_REQUIREMENT),
            AccessLevel::Expert => Some(EXPERT_REQUIREMENT),
            _ => None,
        }
    }
}

impl Account {
    /// Determines the [`AccessLevel`] of the account based on its current level and karma.
    ///
    /// Access levels are determined by the account's total `level` and the `karma30` (karma earned
    /// in the last 30 days). This means an account's access level can dynamically change
    /// (potentially decrease) based on recent activity, except for accounts listed in
    /// [`Config::protoadmin_ids`], which always retain [`AccessLevel::Protoadmin`] privileges.
    ///
    /// This method relies on [`Account::id`], [`Account::level`] and [`Account::karma30`] fields
    /// being set.
    #[must_use]
    pub fn access_level(&self, config: &Config) -> AccessLevel {
        // NOTE: Linear `.contains()` search is preferred for small vectors, so there's no overhead
        if config.protoadmin_ids.contains(&self.id) {
            AccessLevel::Protoadmin
        } else {
            AccessLevel::iter()
                .rev()
                .find(|access_level| {
                    let requirement = access_level.requirement();
                    requirement.is_some_and(|requirement| {
                        self.level >= requirement.0 && self.karma30 >= requirement.1
                    })
                })
                .unwrap_or(AccessLevel::User)
        }
    }
}
