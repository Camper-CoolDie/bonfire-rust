use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::account::EffectKind;
use crate::models::publication::Kind as PublicationKind;
use crate::models::{self, AccountRef, ChatTag, Effect, FandomRef, Gender};

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Account {
    EffectApplied {
        effect: Effect,
        admin_name: String,
        admin_gender: Gender,
    },
    EffectRemoved {
        id: u64,
        kind: EffectKind,
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    FandomUnbanned {
        fandom: FandomRef,
        moderator: AccountRef,
        reason: String,
    },
    Followed {
        account: AccountRef,
    },
    Mentioned {
        account: AccountRef,
        publication_id: u64,
        publication_kind: PublicationKind,
        chat_tag: Option<ChatTag>,
        text: String,
    },
    Punished {
        reason: String,
        banned_until: Option<DateTime<Utc>>,
    },
    PunishmentRemoved {
        admin: AccountRef,
        reason: String,
    },
    TargetAdminActionRejected {
        // TODO: admin actions
        // action: AdminAction,
        rejected_by: models::Account,
        created_by: models::Account,
        target: models::Account,
        reason: String,
    },
    Unfollowed {
        account: AccountRef,
    },
}
