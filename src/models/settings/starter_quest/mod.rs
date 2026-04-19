mod stage;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub use stage::Stage;

/// Represents the user's starter quest progress.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct StarterQuest {
    /// The current stage of the starter quest
    pub stage: Stage,
    /// The current progress within the current stage
    pub progress: usize,
}

impl StarterQuest {
    /// Returns the maximum progress required to complete the current stage, or `None` if the
    /// stage is unknown.
    #[must_use]
    pub fn max_progress(&self) -> Option<usize> {
        self.stage.max_progress()
    }
}
