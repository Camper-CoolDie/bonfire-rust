use chrono::{DateTime, Utc};

/// Represents the type of an effect applied to an account.
#[derive(Default, Clone, Debug)]
pub enum EffectKind {
    /// This user cannot place negative rates
    #[default]
    Hater,
    /// The user's avatar is replaced with a pig image
    Pig,
    /// This user cannot block publications
    Watchman,
    /// A persistent goose runs across the user's screen
    Goose,
    /// The user experiences a constant snowing animation
    EternalWinter,
    /// This user is temporarily restricted from performing administrative actions
    Punished,
    /// This user has privileges to translate the application regardless of their level and karma
    Translator,
    /// This user cannot mention others using the "@" symbol
    MentionLock,
}

/// Represents a preselected reason for an effect being applied.
#[derive(Default, Clone, Debug)]
pub enum EffectReasonKind {
    /// The reason for the effect is not preselected
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
    /// Punished for placing negative rates on every publication seen
    Hater,
    /// Punished for being uncultured
    Uncultured,
}

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
    pub reason_kind: EffectReasonKind,
    /// The name of the account that applied this effect (`None` if
    /// [`is_system`][Effect::is_system] is true)
    pub from_account_name: Option<String>,
}
