/// Represents a reaction on a publication.
#[derive(Default, Clone, Debug)]
pub struct Reaction {
    /// An identifier of an account who placed this reaction
    pub from_account_id: u64,
    /// An index of this reaction
    pub index: i64,
}
