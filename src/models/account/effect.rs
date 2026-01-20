use chrono::{DateTime, Utc};

/// Represents an effect type.
#[derive(Default, Clone, Debug)]
pub enum EffectKind {
    /// This user can't place negative rates
    #[default]
    Hater,
    /// An avatar of this user is replaced with a pig
    Pig,
    /// This user can't block publications
    Watchman,
    /// A goose is running across the entire screen of this user which they can't get rid of
    Goose,
    /// It is constantly snowing for this user
    EternalWinter,
    /// This user can't perform admin actions
    Punished,
    /// This user has the privilege to translate the application regardless of their level and
    /// karma
    Translator,
    /// This user can't mention others by using "@"
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
    /// An identifier of an account to whom this effect is applied
    pub account_id: u64,
    /// The date when this effect was applied
    pub applied_at: DateTime<Utc>,
    /// The effect's end date
    pub ends_at: DateTime<Utc>,
    /// A reason for applying this effect. `None` if [`is_system`][Effect::is_system] is true
    pub reason: Option<String>,
    /// A type of this effect
    pub kind: EffectKind,
    /// Was this effect applied by the system?
    pub is_system: bool,
    /// A preselected reason for applying this effect. Useful only if
    /// [`is_system`][Effect::is_system] is true, otherwise use the [`reason`][Effect::reason]
    /// field
    pub reason_kind: EffectReasonKind,
    /// A name of an account who applied this effect. `None` if [`is_system`][Effect::is_system] is
    /// true
    pub from_account_name: Option<String>,
}
