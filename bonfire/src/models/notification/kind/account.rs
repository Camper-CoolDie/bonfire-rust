use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::account::EffectKind;
use crate::models::notification::{Kind, Notifiable};
use crate::models::publication::Kind as PublicationKind;
use crate::models::{Account as AccountModel, AccountRef, ChatTag, Effect, FandomRef, Gender};
use crate::sealed::Sealed;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Account {
    EffectApplied {
        effect: Effect,
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
        banned_until: Option<DateTime<Utc>>,
        reason: String,
    },
    PunishmentRemoved {
        admin: AccountRef,
        reason: String,
    },
    TargetAdminActionRejected {
        // TODO: admin actions
        // action: AdminAction,
        rejected_by: AccountModel,
        created_by: AccountModel,
        reason: String,
    },
    Unfollowed {
        account: AccountRef,
    },
}

impl Notifiable for Account {
    fn kind(&self) -> Kind {
        match self {
            Account::EffectApplied { .. } => Kind::EffectApplied,
            Account::EffectRemoved { .. } => Kind::EffectRemoved,
            Account::FandomUnbanned { .. } => Kind::AccountFandomUnbanned,
            Account::Followed { .. } => Kind::AccountFollowed,
            Account::Mentioned { .. } => Kind::AccountMentioned,
            Account::Punished { .. } => Kind::AccountPunished,
            Account::PunishmentRemoved { .. } => Kind::PunishmentRemoved,
            Account::TargetAdminActionRejected { .. } => Kind::AccountTargetAdminActionRejected,
            Account::Unfollowed { .. } => Kind::AccountUnfollowed,
        }
    }
}

impl Sealed for Account {}
