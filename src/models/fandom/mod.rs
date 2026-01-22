use chrono::{DateTime, Utc};

use crate::models::{Category, ImageRef, Language};

/// Represents the current status of a fandom.
#[derive(Default, Clone, Debug)]
pub enum FandomStatus {
    /// The fandom's status is unspecified
    #[default]
    Unspecified,
    /// The fandom has been suggested and is awaiting approval
    Suggested,
    /// The fandom has been accepted
    Accepted,
}

/// Represents a fandom, which is a community centered around a specific topic.
#[derive(Default, Clone, Debug)]
pub struct Fandom {
    /// The unique identifier of this fandom
    pub id: u64,
    /// The specific language for this fandom. A single fandom can exist in multiple languages,
    /// with each `Language` representing a distinct community instance. `None` if this fandom
    /// instance isn't linked to a specific language
    pub language: Option<Language>,
    /// The icon image representing this fandom
    pub icon: Option<ImageRef>,
    /// The background image set for this fandom's profile
    pub background: Option<ImageRef>,
    /// The GIF background image set for this fandom's profile
    pub background_gif: Option<ImageRef>,
    /// Indicates if publications from this fandom can randomly appear in the feed
    pub is_closed: bool,
    /// The karma coefficient applied to this fandom
    pub karma_coef: f64,
    /// The identifier of the account that originally suggested this fandom
    pub suggester_id: Option<u64>,
    /// The date and time when this fandom was suggested
    pub suggested_at: Option<DateTime<Utc>>,
    /// The number of users currently subscribed to this fandom
    pub subscribers_count: u64,
    /// The current status of this fandom
    pub status: FandomStatus,
    /// The category that this fandom belongs to
    pub category: Category,
}
