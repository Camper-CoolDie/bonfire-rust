/// Represents the current status of a fandom.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Status {
    /// The fandom has been suggested and is awaiting approval
    Suggested,
    /// The fandom has been accepted
    Accepted,
}
