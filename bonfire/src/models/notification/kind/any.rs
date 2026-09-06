#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Account, Chat, Fandom, Post, Profile, Publication, Rubric};
use crate::models::notification::{Kind, Notifiable};
use crate::models::publication::Kind as PublicationKind;
use crate::models::{Account as AccountModel, AccountRef, Gender, Language};
use crate::sealed::Sealed;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum AnyNotification {
    Account(Account),
    Chat(Chat),
    Fandom(Fandom),
    Post(Post),
    Profile(Profile),
    Publication(Publication),
    Rubric(Rubric),
    AdminActionRejected {
        // TODO: admin actions
        // action: AdminAction,
        rejected_by: AccountModel,
        created_by: AccountModel,
        reason: String,
    },
    BlockRejected {
        moderation_id: u64,
        fandom_id: u64,
        fandom_language: Language,
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    CommentReplied {
        id: u64,
        author: AccountRef,
        parent_id: u64,
        parent_kind: PublicationKind,
        text: Option<String>,
    },
    DonationProcessed {
        amount: u64,
    },
    #[cfg_attr(feature = "serde", serde(untagged))]
    Unknown(i64),
}

impl Notifiable for AnyNotification {
    fn kind(&self) -> Kind {
        match self {
            AnyNotification::Account(account) => account.kind(),
            AnyNotification::Chat(chat) => chat.kind(),
            AnyNotification::Fandom(fandom) => fandom.kind(),
            AnyNotification::Post(post) => post.kind(),
            AnyNotification::Profile(profile) => profile.kind(),
            AnyNotification::Publication(publication) => publication.kind(),
            AnyNotification::Rubric(rubric) => rubric.kind(),
            AnyNotification::AdminActionRejected { .. } => Kind::AdminActionRejected,
            AnyNotification::BlockRejected { .. } => Kind::BlockRejected,
            AnyNotification::CommentReplied { .. } => Kind::CommentReplied,
            AnyNotification::DonationProcessed { .. } => Kind::DonationProcessed,
            AnyNotification::Unknown(kind) => Kind::Unknown(*kind),
        }
    }
}

impl Sealed for AnyNotification {}
