/// Represents a role of a member within a group chat.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum MemberRole {
    /// A regular chat member
    #[default]
    User,
    /// A chat moderator
    Moderator,
    /// A chat administrator
    Admin,
}
