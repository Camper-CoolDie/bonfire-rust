#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::ImageRef;

/// The maximum allowed size in bytes for a static comment image.
pub const COMMENT_IMAGE_MAX_SIZE: usize = 256 * 1024;
/// The maximum allowed dimension (width or height) for a static comment image.
pub const COMMENT_IMAGE_MAX_DIMENSION: usize = 1080;
/// The maximum allowed size in bytes for a GIF comment.
pub const COMMENT_GIF_MAX_SIZE: usize = 1024 * 1024;
/// The maximum allowed dimension (width or height) for a GIF comment.
pub const COMMENT_GIF_MAX_DIMENSION: usize = 400;
/// The maximum number of static images that can be included in a single comment.
pub const COMMENT_IMAGES_MAX_COUNT: usize = 5;

/// Represents the rich media content of a comment.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Content {
    /// No specific content
    #[default]
    Text,
    /// A single static image
    Image(ImageRef),
    /// An animated GIF image
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
        /// The GIF representation of the sticker, if available
        gif: Option<ImageRef>,
    },
    /// An unknown content type
    Unknown(i64),
}
