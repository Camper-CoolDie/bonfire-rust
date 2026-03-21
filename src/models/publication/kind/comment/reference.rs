use super::Content;

/// Represents a reference within a comment to another comment.
///
/// This structure links a comment to the content it is replying to, providing context.
#[derive(Default, Clone, Debug)]
pub struct Reference {
    /// The unique identifier of the referenced comment
    pub id: u64,
    /// The content of the referenced comment
    pub content: Content,
    /// A snippet of the text from the referenced comment, if applicable
    pub text: Option<String>,
    /// The name of the author of the referenced comment
    pub author_name: String,
}
