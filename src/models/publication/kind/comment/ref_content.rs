#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::ImageRef;

/// Represents the rich media content of a referenced comment.
#[derive(Default, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
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
}
