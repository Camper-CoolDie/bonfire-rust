mod color;

pub use color::Color;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the visual theme selected by the user.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Theme {
    /// The theme automatically adjusts based on the system's theme settings.
    ///
    /// The provided `Color` specifies the accent color for this theme.
    SystemDependent(Color),
    /// A light theme.
    ///
    /// The provided `Color` specifies the accent color for this theme.
    Light(Color),
    /// A dim theme
    Dim,
    /// A dark theme.
    ///
    /// The provided `Color` specifies the accent color for this theme.
    Dark(Color),
    /// An unknown or unrecognized theme variant
    Unknown(i32),
}

impl Default for Theme {
    fn default() -> Self {
        Self::SystemDependent(Color::default())
    }
}
