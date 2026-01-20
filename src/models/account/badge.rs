use crate::models::ImageRef;

/// Represents a badge inside a profile.
#[derive(Default, Clone, Debug)]
pub struct Badge {
    /// An index of this badge
    pub index: i64,
    /// A unique image of this badge
    pub image: ImageRef,
}
