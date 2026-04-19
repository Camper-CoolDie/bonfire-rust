#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents the preferred user interface layout kind.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum InterfaceKind {
    /// A navigation panel-based interface layout
    #[default]
    NavigationPanel,
    /// A menu-based interface layout
    Menu,
}
