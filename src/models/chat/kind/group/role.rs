#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a role of a member within a group chat.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum MemberRole {
    /// A regular chat member
    #[default]
    User,
    /// A chat moderator
    Moderator,
    /// A chat administrator
    Admin,
}
