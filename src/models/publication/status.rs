#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the current status of a publication.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Status {
    /// The publication is a draft, not yet published
    #[default]
    Draft,
    /// The publication has been published
    Published,
    /// The publication has been blocked
    Blocked,
    /// The publication has been deep-blocked (cannot be reverted without protoadmin privileges)
    DeepBlocked,
    /// The publication is scheduled to be published at a future date by the author
    Pending,
}
