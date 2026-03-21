use crate::models::ImageRef;

/// Represents the rich media content of a comment or referenced publication.
#[derive(Default, Clone, Debug)]
pub enum Content {
    /// No specific content
    #[default]
    Text,
    /// A single static image
    Image(ImageRef),
    /// An animated GIF image. This variant can only be directly constructed as
    /// [`Comment::content`][crate::models::Comment::content]
    Gif {
        /// The first frame of the GIF as a static image
        first_frame: ImageRef,
        /// The animated GIF itself
        animated: ImageRef,
    },
    /// A collection of multiple static images
    Images(Vec<ImageRef>),
    /// A sticker
    Sticker {
        /// The unique identifier of the sticker
        id: u64,
        /// The static image representation of the sticker
        image: ImageRef,
        /// The GIF representation of the sticker, if available. This will always be `None` when
        /// embedded within a [`CommentReference`][crate::models::publication::CommentReference]
        gif: Option<ImageRef>,
    },
    /// An unknown content type
    Unknown(i64),
}
