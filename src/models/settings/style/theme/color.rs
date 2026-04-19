#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a predefined accent color for themes.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Color {
    /// The default accent color
    #[default]
    Default,
    /// Red accent color
    Red,
    /// Pink accent color
    Pink,
    /// Purple accent color
    Purple,
    /// Deep purple accent color
    DeepPurple,
    /// Indigo accent color
    Indigo,
    /// Blue accent color
    Blue,
    /// Light blue accent color
    LightBlue,
    /// Cyan accent color
    Cyan,
    /// Teal accent color
    Teal,
    /// Green accent color
    Green,
    /// Light green accent color
    LightGreen,
    /// Lime accent color
    Lime,
    /// Deep orange accent color
    DeepOrange,
    /// Brown accent color
    Brown,
    /// Blue-gray accent color
    BlueGray,
}
