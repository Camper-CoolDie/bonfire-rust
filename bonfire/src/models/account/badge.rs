#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::ImageRef;

/// Represents a badge that can be displayed within a user's profile.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Badge {
    /// The index of this badge
    pub index: i64,
    /// The image representing this badge
    pub image: ImageRef,
}
