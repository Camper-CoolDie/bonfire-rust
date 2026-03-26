use crate::models::ImageRef;

/// Represents the content of a referenced chat message.
#[derive(Default, Clone, Debug)]
pub enum RefContent {
    /// No specific content
    #[default]
    Text,
    /// A single static image
    Image(ImageRef),
    /// A collection of multiple static images
    Images(Vec<ImageRef>),
    /// A sticker
    Sticker {
        /// The unique identifier of the sticker
        id: u64,
        /// The static image representation of the sticker
        image: ImageRef,
    },
    /// A voice message
    Voice,
}
