/// Represents the current status of a publication.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PublicationStatus {
    /// The publication is a draft, not yet published
    Draft,
    /// The publication has been published
    Published,
    /// The publication has been blocked
    Blocked,
    /// The publication has been deep-blocked (cannot be reverted without protoadmin privileges)
    DeepBlocked,
    /// The publication is scheduled to be published at a future date by the author
    Pending,
}
