#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::models::{Account, Config};

pub enum Requirement {
    None,
    Level { level: f64, karma30: Option<f64> },
    Other,
}
impl Requirement {
    pub const fn new(level: f64, karma30: Option<f64>) -> Self {
        Self::Level { level, karma30 }
    }
}

/// Represents the various access levels an account can hold.
///
/// These levels are hierarchical, meaning higher levels implicitly include privileges of lower
/// levels. The `Default` level is [`AccessLevel::User`].
///
/// Each access level (except for [`AccessLevel::User`] and [`AccessLevel::Protoadmin`]) has
/// specific `level` and `karma30` requirements that an account must meet to attain that level.
#[derive(Default, Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, EnumIter)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum AccessLevel {
    /// Standard user level, the default for all accounts
    #[default]
    User,
    /// Trusted user level
    Trusted,
    /// Experienced user level
    Experienced,
    /// Curator level
    Curator,
    /// Moderator level
    Moderator,
    /// Administrator level
    Admin,
    /// Superadministrator level
    Superadmin,
    /// Expert user level
    Expert,
    /// Protoadministrator level, reserved for accounts explicitly listed in
    /// [`Config::protoadmin_ids`]
    Protoadmin,
}
impl AccessLevel {
    /// Returns the minimum requirement for this access level.
    ///
    /// Returns [`Requirement::None`] for [`AccessLevel::User`] and [`Requirement::Other`] for
    /// [`AccessLevel::Protoadmin`] as they don't have specific dynamic requirements.
    /// [`Requirement::Level`] is returned for other access levels.
    #[must_use]
    pub const fn requirement(self) -> Requirement {
        match self {
            AccessLevel::User => Requirement::None,
            AccessLevel::Trusted => Requirement::new(3.0, None),
            AccessLevel::Experienced => Requirement::new(4.5, Some(150.0)),
            AccessLevel::Curator => Requirement::new(6.0, Some(400.0)),
            AccessLevel::Moderator => Requirement::new(7.0, Some(500.0)),
            AccessLevel::Admin => Requirement::new(8.5, Some(700.0)),
            AccessLevel::Superadmin => Requirement::new(10.0, Some(1000.0)),
            AccessLevel::Expert => Requirement::new(12.0, Some(1300.0)),
            AccessLevel::Protoadmin => Requirement::Other,
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
        AccessLevel::iter()
            .rev()
            .find(|access_level| {
                match access_level.requirement() {
                    Requirement::None => true,
                    Requirement::Level { level, karma30 } => {
                        self.level >= level && karma30.is_none_or(|karma30| self.karma30 >= karma30)
                    }
                    // Linear `.contains()` search is preferred for small vectors, so there's no
                    // overhead
                    Requirement::Other => config.protoadmin_ids.contains(&self.id),
                }
            })
            .expect("failed to determine access level")
    }
}
