/// Represents the status of a member within a group chat.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum MemberStatus {
    /// The member is currently in the chat
    #[default]
    Active,
    /// The member has left the chat
    Left,
    /// The member was removed from the chat by an administrator or moderator
    Removed,
    /// The member both left the chat and was subsequently removed
    LeftAndRemoved,
}
