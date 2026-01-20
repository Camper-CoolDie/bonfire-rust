use chrono::{DateTime, Utc};

use crate::models::{Category, ImageRef, Language};

/// Represents a fandom status.
#[derive(Default, Clone, Debug)]
pub enum FandomStatus {
    /// The status is unspecified
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
    /// A language which the fandom's community is speaking in. This field isn't linked to a
    /// separate fandom but further identifies it. Should be set in most cases if constructing with
    /// `{ ... }`. `None` if multilingual
    pub language: Option<Language>,
    /// An avatar of this fandom
    pub icon: Option<ImageRef>,
    /// A background of this fandom
    pub background: Option<ImageRef>,
    /// A GIF background of this fandom
    pub background_gif: Option<ImageRef>,
    /// Can publications from this fandom randomly appear in feed?
    pub is_closed: bool,
    /// A karma coefficient of this fandom
    pub karma_coef: f64,
    /// An identifier of an account who suggested this fandom
    pub suggester_id: Option<u64>,
    /// The date when this fandom was suggested
    pub suggested_at: Option<DateTime<Utc>>,
    /// The number of users who subscribed to this fandom
    pub subscribers_count: u64,
    /// A status of this fandom
    pub status: FandomStatus,
    /// A category of this fandom
    pub category: Category,
}
