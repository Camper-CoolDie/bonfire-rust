/// Represents a preselected reason for an effect being applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectReasonKind {
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
