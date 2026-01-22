/// Represents a reference to an external image, including its metadata.
#[derive(Default, Clone, Debug)]
pub struct ImageRef {
    /// The unique identifier of this image
    pub id: u64,
    /// The URI from which this image can be downloaded
    pub uri: String,
    /// The width of the image in pixels; can be zero if unknown
    pub width: u32,
    /// The height of the image in pixels; can be zero if unknown
    pub height: u32,
}
