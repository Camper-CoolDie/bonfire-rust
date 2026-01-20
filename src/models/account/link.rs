/// Represents a link inside a profile.
#[derive(Default, Clone, Debug)]
pub struct Link {
    /// A title of this link
    pub title: String,
    /// A URI which this link is leading to
    pub uri: String,
}
