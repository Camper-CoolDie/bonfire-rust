use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::AccountRef;
use crate::models::notification::{Kind, Notifiable};
use crate::models::publication::{Kind as PublicationKind, PostTitle};
use crate::sealed::Sealed;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Publication {
    Blocked {
        publication_kind: PublicationKind,
        moderation_id: u64,
        with_last_publications: bool,
        is_punished: bool,
        banned_until: Option<DateTime<Utc>>,
        reason: String,
    },
    BlockedAfterReport {
        publication_kind: PublicationKind,
        moderation_id: u64,
        with_last_publications: bool,
        is_punished: bool,
        banned_until: Option<DateTime<Utc>>,
        reason: String,
    },
    Commented {
        id: u64,
        author: AccountRef,
        fandom_name: String,
        parent_id: u64,
        parent_kind: PublicationKind,
        parent_author_id: u64,
        parent_post_title: Option<PostTitle>,
        text: Option<String>,
    },
    Rated {
        amount: f64,
        account: Option<AccountRef>,
        publication_id: u64,
        publication_kind: PublicationKind,
        post_title: Option<PostTitle>,
        parent_id: u64,
        parent_kind: PublicationKind,
    },
    Reacted {
        index: i64,
        account: AccountRef,
        publication_id: u64,
        publication_kind: PublicationKind,
        parent_id: u64,
        parent_kind: PublicationKind,
    },
    Restored {
        id: u64,
        kind: PublicationKind,
        parent_id: u64,
        parent_kind: PublicationKind,
        reason: String,
    },
}

impl Notifiable for Publication {
    fn kind(&self) -> Kind {
        match self {
            Publication::Blocked { .. } => Kind::PublicationBlocked,
            Publication::BlockedAfterReport { .. } => Kind::PublicationBlockedAfterReport,
            Publication::Commented { .. } => Kind::PublicationCommented,
            Publication::Rated { .. } => Kind::PublicationRated,
            Publication::Reacted { .. } => Kind::PublicationReacted,
            Publication::Restored { .. } => Kind::PublicationRestored,
        }
    }
}

impl Sealed for Publication {}
