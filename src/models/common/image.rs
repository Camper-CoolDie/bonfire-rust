/// Represents a reference to an external image, including its metadata.
#[derive(Default, Clone, Debug)]
pub struct ImageRef {
    /// The unique identifier of this image
    pub id: u64,
    /// The URI from which this image can be downloaded
    pub uri: Option<String>,
    /// The width of the image in pixels; can be zero if unknown
    pub width: usize,
    /// The height of the image in pixels; can be zero if unknown
    pub height: usize,
}
