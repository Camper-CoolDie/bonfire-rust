#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::notification::{Kind, Notifiable};
use crate::models::publication::PostTitle;
use crate::models::{AccountRef, FandomRef, Gender};
use crate::sealed::Sealed;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Post {
    Closed {
        moderation_id: u64,
        moderator_name: String,
        moderator_gender: Gender,
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
        moderator_name: String,
        moderator_gender: Gender,
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

impl Notifiable for Post {
    fn kind(&self) -> Kind {
        match self {
            Post::Closed { .. } => Kind::PostClosed,
            Post::Drafted { .. } => Kind::PostDrafted,
            Post::FandomChanged { .. } => Kind::PostFandomChanged,
            Post::FollowedPostCreated { .. } => Kind::FollowedPostCreated,
            Post::ImagesPurged { .. } => Kind::PostImagesPurged,
            Post::ImportantPostCreated { .. } => Kind::ImportantPostCreated,
            Post::MultilingualDisabled { .. } => Kind::PostMultilingualDisabled,
            Post::NsfwToggled { .. } => Kind::PostNsfwToggled,
            Post::Opened { .. } => Kind::PostOpened,
            Post::RelayPostCreated { .. } => Kind::PostRelayPostCreated,
            Post::RelayTurnAssigned { .. } => Kind::PostRelayTurnAssigned,
            Post::RelayTurnMissed { .. } => Kind::PostRelayTurnMissed,
            Post::RelayTurnRejected { .. } => Kind::PostRelayTurnRejected,
            Post::TagsChanged { .. } => Kind::PostTagsChanged,
        }
    }
}

impl Sealed for Post {}
