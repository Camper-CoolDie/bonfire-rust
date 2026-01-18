use chrono::{DateTime, Utc};

/// Represents an effect type.
#[derive(Default, Clone, Debug)]
pub enum EffectKind {
    /// The user can't place negative rates
    #[default]
    Hater,
    /// The user's avatar is replaced with a pig
    Pig,
    /// The user can't block publications
    Watchman,
    /// A goose is running across the user's screen which you can't get rid of
    Goose,
    /// It is constantly snowing for the user
    EternalWinter,
    /// The user can't perform admin actions
    Punished,
    /// The user has the privilege to translate the application regardless of their level and karma
    Translator,
    /// The user can't mention others by using "@"
    MentionLock,
}

/// Represents a preselected reason kind of an effect.
#[derive(Default, Clone, Debug)]
pub enum EffectReasonKind {
    /// The reason is not preselected
    #[default]
    None,
    /// Punished for inappropriate behavior towards the gods
    Gods,
    /// Punished for unreasonable blocks
    RejectedBlocks,
    /// Punished for blocking too many publications
    TooManyBlocks,
    /// Punished for swearing in the service
    Swearing,
    /// Punished for placing negative rates to every publication they saw
    Hater,
    /// Punished for being uncultured
    Uncultured,
}

/// Represents an account effect.
#[derive(Default, Clone, Debug)]
pub struct Effect {
    /// A unique identifier of this effect. Should always be set to a valid value if constructing
    /// with `{ ... }`
    pub id: u64,
    /// An account identifier to whom this effect is applied
    pub account_id: u64,
    /// The date when this effect was applied
    pub applied_at: DateTime<Utc>,
    /// The effect's end date
    pub ends_at: DateTime<Utc>,
    /// A reason for applying this effect. Empty if `is_system` is true
    pub reason: String,
    /// The effect's type
    pub kind: EffectKind,
    /// Was this effect applied by the system?
    pub is_system: bool,
    /// A preselected reason for applying this effect. Useful only if `is_system` is true,
    /// otherwise use `reason`
    pub reason_kind: EffectReasonKind,
    /// An account name who applied this effect. Empty if `is_system` is true
    pub from_account_name: String,
}
