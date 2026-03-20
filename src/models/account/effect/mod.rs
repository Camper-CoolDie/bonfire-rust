mod kind;
mod reason_kind;

use chrono::{DateTime, Utc};
pub use kind::EffectKind;
pub use reason_kind::EffectReasonKind;

/// Represents an effect applied to an account.
#[derive(Default, Clone, Debug)]
pub struct Effect {
    /// The unique identifier of this effect
    pub id: u64,
    /// The identifier of the account to whom this effect is applied
    pub account_id: u64,
    /// The date and time when this effect was applied
    pub applied_at: DateTime<Utc>,
    /// The date and time when this effect is scheduled to end
    pub ends_at: DateTime<Utc>,
    /// The reason for applying this effect (`None` if [`is_system`][Effect::is_system] is true)
    pub reason: Option<String>,
    /// The type of this effect
    pub kind: EffectKind,
    /// Indicates if this effect was applied automatically by the system
    pub is_system: bool,
    /// A preselected reason for applying this effect ([`reason`][Effect::reason] should be used
    /// otherwise)
    pub reason_kind: Option<EffectReasonKind>,
    /// The name of the account that applied this effect (`None` if
    /// [`is_system`][Effect::is_system] is true)
    pub from_account_name: Option<String>,
}
