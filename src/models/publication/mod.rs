mod kind;
mod reaction;

use chrono::{DateTime, Utc};
pub use kind::*;
pub use reaction::Reaction;

use crate::models::{Account, Category, Fandom};

/// A trait for publication types that extend the core [`Publication`] struct.
///
/// This trait allows for adding type-specific fields beyond the generic [`Publication`] data.
/// [`AnyPublication`] serves as a catch-all for publications with unspecified specific types.
pub trait PublicationInheritor {
    /// Returns the specific kind of this publication.
    fn kind(&self) -> PublicationKind;
}

/// Represents the current status of a publication.
#[derive(Default, Clone, Debug)]
pub enum PublicationStatus {
    /// The publication status is unspecified
    #[default]
    Unspecified,
    /// The publication is a draft, not yet published
    Draft,
    /// The publication has been published
    Published,
    /// The publication has been blocked
    Blocked,
    /// The publication has been deep-blocked (cannot be reverted without protoadmin privileges)
    DeepBlocked,
    /// The publication is scheduled to be published at a future date by the author
    Pending,
}

/// Represents a generic Bonfire publication.
#[derive(Default, Clone, Debug)]
pub struct Publication<T: PublicationInheritor = AnyPublication> {
    /// The unique identifier of this publication
    pub id: u64,
    /// Additional, type-specific data for this publication
    pub kind: T,
    /// The fandom in which this publication was posted
    pub fandom: Fandom,
    /// The account that authored this publication
    pub author: Account,
    /// The specific category of the fandom in which this publication was posted
    pub category: Category,
    /// The date and time when this publication was created (or published, for posts/quests)
    pub created_at: DateTime<Utc>,
    /// The identifier of the parent publication, if this is a reply or sticker
    pub parent_id: Option<u64>,
    /// The type of the parent publication, if applicable
    pub parent_kind: Option<PublicationKind>,
    /// The total karma received by this publication (can be positive or negative)
    pub karma: f64,
    /// The karma you personally placed on this publication, if any
    pub my_karma: Option<f64>,
    /// The current status of this publication
    pub status: PublicationStatus,
    /// Indicates if this publication will appear in the main feed
    pub is_closed: bool,
    /// The total number of comments associated with this publication
    pub comments_count: u64,
    /// Indicates if this publication has been marked as important
    pub is_important: bool,
    /// Indicates if this publication originates from a blocked fandom or account
    pub is_hidden: bool,
    /// Indicates if this publication has been marked as NSFW
    pub is_nsfw: bool,
    /// A value indicating the "hotness" or popularity of this publication
    pub hotness: f32,
}
