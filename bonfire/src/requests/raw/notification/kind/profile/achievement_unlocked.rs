use serde::Deserialize;

use crate::models::notification::AchievementUnlocked;

#[derive(Deserialize)]
pub(crate) struct RawAchievementUnlocked {}

impl From<RawAchievementUnlocked> for AchievementUnlocked {
    fn from(_value: RawAchievementUnlocked) -> Self {
        Self {}
    }
}
