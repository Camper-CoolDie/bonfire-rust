mod kind;
mod reaction;
mod status;

use chrono::{DateTime, Utc};
pub use kind::*;
pub use reaction::Reaction;
pub use status::Status;

use crate::sealed::Sealed;

/// A trait for publication types that extend the core [`Publication`] struct.
///
/// This trait allows for adding type-specific fields beyond the generic [`Publication`] data.
/// [`AnyPublication`] serves as a catch-all for publications with unspecified specific types.
pub trait Publishable: Sealed {
    /// Returns the specific kind of this publication.
    fn kind(&self) -> Kind;
}

/// Represents a generic Bonfire publication.
#[derive(Default, Clone, Debug)]
pub struct Publication<T: Publishable = AnyPublication> {
    /// The unique identifier of this publication
    pub id: u64,
    /// Additional, type-specific data for this publication
    pub kind: T,
    /// The date and time when this publication was created (or published, for posts/quests)
    pub created_at: DateTime<Utc>,
    /// The current status of this publication, or `None` if unspecified
    pub status: Option<Status>,
    /// A value indicating the "hotness" or popularity of this publication
    pub hotness: f32,
}
