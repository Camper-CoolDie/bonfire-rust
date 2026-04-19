#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a preselected reason for an effect being applied.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum ReasonKind {
    /// Punished for inappropriate behavior towards the gods
    #[default]
    Gods,
    /// Punished for unreasonable blocks
    RejectedBlocks,
    /// Punished for blocking too many publications
    TooManyBlocks,
    /// Punished for swearing in the service
    Swearing,
    /// Punished for placing negative rates on every publication seen
    Hater,
    /// Punished for being uncultured
    Uncultured,
    /// An unknown reason
    Unknown(i64),
}
