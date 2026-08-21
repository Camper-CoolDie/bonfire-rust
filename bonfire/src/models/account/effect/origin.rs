#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::ReasonKind;

/// Represents the origin or source of an [`Effect`][super::Effect].
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Origin {
    /// The effect was applied by another account
    Account {
        /// The name of the account that applied the effect
        name: String,
        /// A specific reason provided by the account for applying the effect
        reason: String,
    },
    /// The effect was applied by the system
    System {
        /// The kind of reason for the system-applied effect
        reason_kind: ReasonKind,
    },
}

impl Default for Origin {
    fn default() -> Self {
        Self::Account {
            name: String::new(),
            reason: String::new(),
        }
    }
}
