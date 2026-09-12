mod filter;
mod kind;

use chrono::{DateTime, Utc};
pub use filter::Filter;
pub use kind::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Notification {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub content: AnyNotification,
    pub sent_at: DateTime<Utc>,
}
