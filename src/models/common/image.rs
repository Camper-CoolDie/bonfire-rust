/// Represents a reference to an external image.
#[derive(Default, Clone, Debug)]
pub struct ImageRef {
    /// A unique identifier of this image.
    pub id: u64,
    /// A URI which can be used to download this image.
    pub uri: String,
    /// The image's width. Can be zero
    pub width: u32,
    /// The image's height. Can be zero
    pub height: u32,
}
