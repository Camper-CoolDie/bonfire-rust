#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the current status of a fandom.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Status {
    /// The fandom has been suggested and is awaiting approval
    #[default]
    Suggested,
    /// The fandom has been accepted
    Accepted,
}
