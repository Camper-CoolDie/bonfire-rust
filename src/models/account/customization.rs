#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Badge;

/// Represents customizable aspects of an account's appearance.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Customization {
    /// The hexadecimal color code for this account's name (e.g., `0xFFFFFF`).
    pub name_color: Option<u32>,
    /// The badge currently selected and displayed for this account.
    pub active_badge: Option<Badge>,
}
