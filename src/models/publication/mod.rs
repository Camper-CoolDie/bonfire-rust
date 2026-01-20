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
    /// The publication's status is unspecified
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
    /// The publication's fandom
    pub fandom: Fandom,
    /// The publication's author
    pub author: Account,
    /// The publication's category
    pub category: Category,
    /// The date when this publication was created (published)
    pub created_at: DateTime<Utc>,
    /// The parent publication's ID (if any)
    pub parent_id: Option<u64>,
    /// The parent publication's type (if any)
    pub parent_kind: Option<PublicationKind>,
    /// The publication's karma amount
    pub karma: f64,
    /// The amount of karma you've placed on this publication
    pub my_karma: Option<f64>,
    /// The publication's status
    pub status: PublicationStatus,
    /// Will this publication appear in feed? (not to be confused with [`Fandom::is_closed`])
    pub is_closed: bool,
    /// The number of comments on this publication
    pub comments_count: u64,
    /// Is this publication marked as important?
    pub is_important: bool,
    /// Does this publication come from a blacklisted fandom or account?
    pub is_blacklisted: bool,
    /// Is this publication marked as Not Safe For Work?
    pub is_nsfw: bool,
    /// The publication's hotness
    pub hotness: f32,
}
