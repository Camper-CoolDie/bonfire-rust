mod kind;
mod origin;
mod reason_kind;

use chrono::{DateTime, Utc};
pub use kind::Kind;
pub use origin::Origin;
pub use reason_kind::ReasonKind;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents an effect applied to an account.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Effect {
    /// The unique identifier of this effect
    pub id: u64,
    /// The identifier of the account to whom this effect is applied
    pub account_id: u64,
    /// The date and time when this effect was applied
    pub applied_at: DateTime<Utc>,
    /// The date and time when this effect is scheduled to end
    pub ends_at: DateTime<Utc>,
    /// The type of this effect
    pub kind: Kind,
    /// The origin or source of this effect
    pub origin: Origin,
}
