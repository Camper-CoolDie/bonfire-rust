use super::RefContent;

/// Represents a reference within a chat message to another publication.
///
/// This structure links a chat message to the content it is replying to, providing context.
#[derive(Default, Clone, Debug)]
pub struct Reference {
    /// The unique identifier of the referenced chat message
    pub id: u64,
    /// The content of the referenced chat message
    pub content: RefContent,
    /// A snippet of the text from the referenced chat message, if applicable
    pub text: Option<String>,
    /// Indicates if the [`text`][Reference::text] field contains a truncated version of the
    /// original text
    pub is_text_truncated: bool,
    /// The name of the author of the referenced chat message
    pub author_name: String,
}
