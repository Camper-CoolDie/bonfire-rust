#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Represents a role of a member within a group chat.
#[derive(Default, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, EnumIter)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum MemberRole {
    /// A regular chat member
    #[default]
    User,
    /// A chat moderator
    Moderator,
    /// A chat administrator
    Admin,
}
