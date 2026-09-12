#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

// TODO:: achievements
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct AchievementUnlocked {}
