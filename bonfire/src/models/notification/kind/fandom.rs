#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::notification::{Kind, Notifiable};
use crate::models::{AccountRef, FandomRef, Gender};
use crate::sealed::Sealed;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Fandom {
    CuratorAssigned {
        old_curator_id: Option<u64>,
        fandom: FandomRef,
        admin: AccountRef,
        reason: String,
    },
    CuratorRevoked {
        fandom: FandomRef,
        admin: AccountRef,
        reason: String,
    },
    ModeratorGranted {
        fandom: FandomRef,
        reason: String,
    },
    ModeratorRevoked {
        fandom: FandomRef,
        reason: String,
    },
    RemovalRejected {
        fandom: FandomRef,
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    Reviewed {
        is_accepted: bool,
        fandom_id: u64,
        fandom_name: String,
        admin_name: String,
        note: String,
    },
}

impl Notifiable for Fandom {
    fn kind(&self) -> Kind {
        match self {
            Fandom::CuratorAssigned { .. } => Kind::FandomCuratorAssigned,
            Fandom::CuratorRevoked { .. } => Kind::FandomCuratorRevoked,
            Fandom::ModeratorGranted { .. } => Kind::FandomModeratorGranted,
            Fandom::ModeratorRevoked { .. } => Kind::FandomModeratorRevoked,
            Fandom::RemovalRejected { .. } => Kind::FandomRemovalRejected,
            Fandom::Reviewed { .. } => Kind::FandomReviewed,
        }
    }
}

impl Sealed for Fandom {}
