/// Represents a reference to an external image.
#[derive(Default, Clone, Debug)]
pub struct ImageRef {
    /// A unique identifier of the image.
    pub id: u64,
    /// A URI to this image.
    pub uri: String,
    /// The image's width. Can be zero
    pub width: u32,
    /// The image's height. Can be zero
    pub height: u32,
}
