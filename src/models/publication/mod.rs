mod kind;
mod reaction;

use chrono::{DateTime, Utc};
pub use kind::*;
pub use reaction::Reaction;

use crate::models::{Account, Category, Fandom};

/// A type of publication which contains other useful fields alongside the main [`Publication`]
/// struct. [`AnyPublication`] is used as a catch-all publication.
pub trait PublicationInheritor {
    /// Get the type of this publication.
    fn kind(&self) -> PublicationKind;
}

/// Represents a publication status.
#[derive(Default, Clone, Debug)]
pub enum PublicationStatus {
    /// The status is unspecified
    #[default]
    Unspecified,
    /// The publication is a draft
    Draft,
    /// The publication is published
    Published,
    /// The publication is blocked
    Blocked,
    /// The publication is deep-blocked (cannot be revert unless you're a protoadmin)
    DeepBlocked,
    /// The publication is waiting to be published
    Pending,
}

/// Represents a publication.
#[derive(Default, Clone, Debug)]
pub struct Publication<T: PublicationInheritor = AnyPublication> {
    /// A unique identifier of this publication. Should always be set to a valid value if
    /// constructing with `{ ... }`
    pub id: u64,
    /// Additional data which depends on a type of this publication
    pub kind: T,
    /// A fandom which this publication was posted in
    pub fandom: Fandom,
    /// An account who made this publication
    pub author: Account,
    /// A category of a fandom which this publication was posted in (the
    /// [`fandom`][Publication::fandom] field will usually have an [`Unknown`][Category::Unknown]
    /// category)
    pub category: Category,
    /// The date when this publication was created (or published, when referring to a post/quest)
    pub created_at: DateTime<Utc>,
    /// An identifier of the parent publication (if any)
    pub parent_id: Option<u64>,
    /// A type of the parent publication (if any)
    pub parent_kind: Option<PublicationKind>,
    /// Total karma placed on this publication, positive or negative
    pub karma: f64,
    /// How much karma you've placed on this publication?
    pub my_karma: Option<f64>,
    /// A status of this publication
    pub status: PublicationStatus,
    /// Will this publication appear in feed? (not to be confused with [`Fandom::is_closed`])
    pub is_closed: bool,
    /// The number of comments this publication has
    pub comments_count: u64,
    /// Was this publication marked as important?
    pub is_important: bool,
    /// Does this publication come from a blacklisted fandom/account?
    pub is_blacklisted: bool,
    /// Was this publication marked as Not Safe For Work?
    pub is_nsfw: bool,
    /// How red is the karma button on this publication
    pub hotness: f32,
}
