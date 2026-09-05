#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, FandomRef, Gender};

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
        fandom: FandomRef,
        admin_name: String,
        note: String,
    },
}
