/// Represents a reaction (emoji) placed on a chat message or comment.
#[derive(Default, Clone, Debug)]
pub struct Reaction {
    /// The index of this reaction
    pub index: i64,
    /// The unique identifier of the account that placed this reaction
    pub from_account_id: u64,
}
