#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Gender;
use crate::models::notification::{Kind, Notifiable};
use crate::sealed::Sealed;

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

impl Notifiable for Profile {
    fn kind(&self) -> Kind {
        match self {
            Profile::AchievementUnlocked => Kind::AchievementUnlocked,
            Profile::DescriptionCleared { .. } => Kind::ProfileDescriptionCleared,
            Profile::LinkRemoved { .. } => Kind::ProfileLinkRemoved,
            Profile::NameCleared { .. } => Kind::ProfileNameCleared,
            Profile::StatusCleared { .. } => Kind::ProfileStatusCleared,
        }
    }
}

impl Sealed for Profile {}
