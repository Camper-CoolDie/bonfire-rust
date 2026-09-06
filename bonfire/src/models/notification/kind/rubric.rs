#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::notification::{Kind, Notifiable};
use crate::models::{AccountRef, FandomRef, Language};
use crate::sealed::Sealed;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Rubric {
    FandomChanged {
        id: u64,
        name: u64,
        moderation_id: u64,
        admin: AccountRef,
        old_fandom: FandomRef,
        new_fandom: FandomRef,
        reason: String,
    },
    KarmaCoefChanged {
        id: u64,
        name: u64,
        fandom_id: u64,
        fandom_language: Language,
        old_coef: f64,
        new_coef: f64,
    },
    NameChanged {
        id: u64,
        old_name: String,
        new_name: String,
        fandom_id: u64,
        fandom_language: Language,
        moderation_id: u64,
        moderator: AccountRef,
        reason: String,
    },
    OwnerAssigned {
        id: u64,
        name: u64,
        fandom_id: u64,
        fandom_language: Language,
        moderation_id: u64,
        moderator: AccountRef,
        reason: String,
    },
    OwnerTransferred {
        id: u64,
        name: u64,
        fandom_id: u64,
        fandom_language: Language,
        new_owner_id: u64,
        new_owner_name: String,
        moderation_id: u64,
        moderator: AccountRef,
        reason: String,
    },
    Removed {
        id: u64,
        name: u64,
        fandom_id: u64,
        fandom_language: Language,
        moderation_id: u64,
        moderator: AccountRef,
        reason: String,
    },
}

impl Notifiable for Rubric {
    fn kind(&self) -> Kind {
        match self {
            Rubric::FandomChanged { .. } => Kind::RubricFandomChanged,
            Rubric::KarmaCoefChanged { .. } => Kind::RubricKarmaCoefChanged,
            Rubric::NameChanged { .. } => Kind::RubricNameChanged,
            Rubric::OwnerAssigned { .. } => Kind::RubricOwnerAssigned,
            Rubric::OwnerTransferred { .. } => Kind::RubricOwnerTransferred,
            Rubric::Removed { .. } => Kind::RubricRemoved,
        }
    }
}

impl Sealed for Rubric {}
