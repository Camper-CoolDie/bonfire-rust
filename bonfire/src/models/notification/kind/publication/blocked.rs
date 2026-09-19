use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::publication::Kind;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PublicationBlocked {
    pub publication_kind: Kind,
    pub moderation_id: u64,
    pub with_last_publications: bool,
    pub is_punished: bool,
    pub banned_until: Option<DateTime<Utc>>,
    pub reason: String,
}
