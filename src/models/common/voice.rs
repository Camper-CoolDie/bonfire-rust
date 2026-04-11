use std::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents a reference to an external audio message, including its metadata.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct VoiceRef {
    /// The unique identifier of the voice message
    pub id: u64,
    /// The URI from which this voice message can be downloaded
    pub uri: String,
    /// The duration of the voice message
    pub duration: Duration,
    /// The waveform data of the voice message
    pub waveform: Vec<u64>,
}
