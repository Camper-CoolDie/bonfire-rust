#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Gender;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Profile {
    // TODO: achievements
    AchievementUnlocked,
    DescriptionCleared {
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    LinkRemoved {
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    NameCleared {
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    StatusCleared {
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
}
