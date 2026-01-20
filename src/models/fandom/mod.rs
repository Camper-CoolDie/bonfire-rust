use chrono::{DateTime, Utc};

use crate::models::{Category, ImageRef, Language};

/// Represents a fandom status.
#[derive(Default, Clone, Debug)]
pub enum FandomStatus {
    /// The fandom's status is unspecified
    #[default]
    Unspecified,
    /// The fandom is waiting for approval
    Suggested,
    /// The fandom was accepted
    Accepted,
}

/// Represents a fandom.
#[derive(Default, Clone, Debug)]
pub struct Fandom {
    /// A unique identifier of this fandom. Should always be set to a valid value if constructing
    /// with `{ ... }`
    pub id: u64,
    /// The fandom's language. Should be set in most cases if constructing with `{ ... }`. `None`
    /// if multilingual
    pub language: Option<Language>,
    /// The fandom's avatar
    pub icon: Option<ImageRef>,
    /// The fandom's background
    pub background: Option<ImageRef>,
    /// The fandom's GIF background
    pub background_gif: Option<ImageRef>,
    /// Will publications from this fandom appear in feed?
    pub is_closed: bool,
    /// The fandom's karma coefficient
    pub karma_coef: f64,
    /// An account identifier who suggested this fandom
    pub suggester_id: Option<u64>,
    /// The date when this fandom was suggested
    pub suggested_at: Option<DateTime<Utc>>,
    /// The number of users who subscribed to this fandom
    pub subscribers_count: u64,
    /// The fandom's status
    pub status: FandomStatus,
    /// The fandom's category
    pub category: Category,
}
