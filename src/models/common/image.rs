/// Represents a reference to an external image.
#[derive(Default, Clone, Debug)]
pub struct ImageRef {
    /// A unique identifier of the image. Zero if the image is unset or default
    pub id: i64,
    /// The image's URI. Empty if the image is unset or default
    pub uri: String,
    /// The image's width. Can be zero
    pub width: i32,
    /// The image's height. Can be zero
    pub height: i32,
}
