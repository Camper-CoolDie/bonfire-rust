use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::AccountRef;
use crate::models::publication::{CommentRefContent, Kind, PostTitle};

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Publication {
    Blocked {
        publication_kind: Kind,
        moderation_id: u64,
        reason: String,
        with_last_publications: bool,
        is_punished: bool,
        banned_until: Option<DateTime<Utc>>,
    },
    BlockedAfterReport {
        publication_kind: Kind,
        moderation_id: u64,
        reason: String,
        with_last_publications: bool,
        is_punished: bool,
        banned_until: Option<DateTime<Utc>>,
    },
    Commented {
        id: u64,
        author: AccountRef,
        fandom_name: String,
        parent_id: u64,
        parent_kind: Kind,
        parent_author_id: u64,
        parent_post_title: Option<PostTitle>,
        text: Option<String>,
        content: CommentRefContent,
    },
    Rated {
        amount: f64,
        account: Option<AccountRef>,
        publication_id: u64,
        publication_kind: Kind,
        post_title: Option<PostTitle>,
        parent_id: u64,
        parent_kind: Kind,
    },
    Reacted {
        index: i64,
        account: AccountRef,
        publication_id: u64,
        publication_kind: Kind,
        post_title: Option<PostTitle>,
        parent_id: u64,
        parent_kind: Kind,
    },
    Restored {
        id: u64,
        kind: Kind,
        parent_id: u64,
        parent_kind: Kind,
        reason: String,
    },
}
