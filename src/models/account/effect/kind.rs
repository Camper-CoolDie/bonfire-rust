#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the type of an effect applied to an account.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Kind {
    /// This user cannot place negative rates
    #[default]
    Hater,
    /// The user's avatar is replaced with a pig image
    Pig,
    /// This user cannot block publications
    Watchman,
    /// A persistent goose runs across the user's screen
    Goose,
    /// The user experiences a constant snowing animation
    EternalWinter,
    /// This user is temporarily restricted from performing administrative actions
    Punished,
    /// This user has privileges to translate the application regardless of their level and karma
    Translator,
    /// This user cannot mention others using the "@" symbol
    MentionLock,
    /// An unknown effect type
    #[cfg_attr(feature = "serde", serde(untagged))]
    Unknown(i64),
}
