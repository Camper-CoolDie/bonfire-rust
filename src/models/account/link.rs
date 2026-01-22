/// Represents an external link displayed within an account's profile.
#[derive(Default, Clone, Debug)]
pub struct Link {
    /// The title or display text for this link
    pub title: String,
    /// The URI this link points to
    pub uri: String,
}
