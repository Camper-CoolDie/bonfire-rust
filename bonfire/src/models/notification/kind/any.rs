#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::{Account, Chat, Fandom, Post, Profile, Publication, Rubric};
use crate::models::publication::{CommentRefContent, Kind as PublicationKind, PostTitle};
use crate::models::{self, AccountRef, Gender, Language};

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
        rejected_by: models::Account,
        created_by: models::Account,
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
        replied_to_comment_id: u64,
        author: AccountRef,
        parent_id: u64,
        parent_kind: PublicationKind,
        parent_post_title: Option<PostTitle>,
        text: Option<String>,
        content: CommentRefContent,
    },
    DonationProcessed {
        amount: u64,
    },
    #[cfg_attr(feature = "serde", serde(untagged))]
    Unknown(i64),
}
