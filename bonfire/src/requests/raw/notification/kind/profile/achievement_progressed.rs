use serde::Deserialize;

use crate::models::notification::AchievementProgressed;

#[derive(Deserialize)]
pub(crate) struct RawAchievementProgressed {}

impl From<RawAchievementProgressed> for AchievementProgressed {
    fn from(_value: RawAchievementProgressed) -> Self {
        Self {}
    }
}
