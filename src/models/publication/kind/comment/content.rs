#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::ImageRef;

/// Represents the rich media content of a comment.
#[derive(Default, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
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
    #[cfg_attr(feature = "serde", serde(untagged))]
    Unknown(i64),
}
impl Content {
    /// The maximum allowed size in bytes for a static comment image.
    pub const IMAGE_MAX_SIZE: usize = 256 * 1024;
    /// The maximum allowed dimension (width or height) for a static comment image.
    pub const IMAGE_MAX_DIMENSION: usize = 1080;
    /// The maximum allowed size in bytes for a GIF comment.
    pub const GIF_MAX_SIZE: usize = 1024 * 1024;
    /// The maximum allowed dimension (width or height) for a GIF comment.
    pub const GIF_MAX_DIMENSION: usize = 400;
    /// The maximum number of static images that can be included in a single comment.
    pub const IMAGES_MAX_COUNT: usize = 5;
}
