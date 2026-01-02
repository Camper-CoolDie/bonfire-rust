use crate::models::ImageRef;

/// Represents a badge in an account's profile.
#[derive(Default, Clone, Debug)]
pub struct Badge {
    /// The badge's index
    pub index: i64,
    /// The badge's image
    pub image: ImageRef,
}
