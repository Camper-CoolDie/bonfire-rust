/// Represents a reaction on a publication.
#[derive(Default, Clone, Debug)]
pub struct Reaction {
    /// The reaction's account ID
    pub from_account_id: i64,
    /// The reaction's index
    pub index: i64,
}
