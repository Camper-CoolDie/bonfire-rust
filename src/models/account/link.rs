/// Represents a link in an account's profile.
#[derive(Default, Clone, Debug)]
pub struct Link {
    /// The link's title
    pub title: String,
    /// The link's URI
    pub uri: String,
}
