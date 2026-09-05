#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::publication::PostTitle;
use crate::models::{AccountRef, FandomRef, Gender};

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Post {
    Closed {
        moderation_id: u64,
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    Drafted {
        title: PostTitle,
        moderation_id: u64,
        moderator_name: String,
        moderator_gender: Gender,
        reason: String,
    },
    FandomChanged {
        post_id: u64,
        old_fandom: FandomRef,
        new_fandom: FandomRef,
        admin: AccountRef,
        reason: String,
    },
    FollowedPostCreated {
        id: u64,
        author: AccountRef,
    },
    ImagesPurged {
        post_id: u64,
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    ImportantPostCreated {
        id: u64,
        fandom: FandomRef,
        importance_moderator_id: u64,
        importance_reason: String,
    },
    MultilingualDisabled {
        moderation_id: u64,
        moderator_name: String,
        moderator_gender: Gender,
        reason: String,
    },
    NsfwToggled {
        is_nsfw: bool,
        moderation_id: u64,
        moderator_name: String,
        moderator_gender: Gender,
        reason: String,
    },
    Opened {
        moderation_id: u64,
        admin_name: String,
        admin_gender: Gender,
        reason: String,
    },
    RelayPostCreated {
        post_id: u64,
        fandom: FandomRef,
        relay_id: u64,
        relay_name: String,
    },
    RelayTurnAssigned {
        account: Option<AccountRef>,
        id: u64,
        name: String,
        fandom: FandomRef,
    },
    RelayTurnMissed {
        id: u64,
        name: String,
        fandom: FandomRef,
        next_account: Option<AccountRef>,
    },
    RelayTurnRejected {
        account: AccountRef,
        id: u64,
        name: String,
        fandom: FandomRef,
        next_account: Option<AccountRef>,
    },
    TagsChanged {
        moderation_id: u64,
        moderator: AccountRef,
        reason: String,
    },
}
