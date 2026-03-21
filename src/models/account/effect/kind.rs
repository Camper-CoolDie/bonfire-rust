/// Represents the type of an effect applied to an account.
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
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
