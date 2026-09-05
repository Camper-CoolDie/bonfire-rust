mod account;
mod any;
mod chat;
mod fandom;
mod post;
mod profile;
mod publication;
mod rubric;

pub use account::Account;
pub use any::AnyNotification;
pub use chat::Chat;
pub use fandom::Fandom;
pub use post::Post;
pub use profile::Profile;
pub use publication::Publication;
pub use rubric::Rubric;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Kind {
    AccountFandomUnbanned,
    AccountFollowed,
    AccountMentioned,
    AccountPunished,
    AccountTargetAdminActionRejected,
    AccountUnfollowed,
    AchievementUnlocked,
    AdminActionRejected,
    BlockRejected,
    ChatMessageCreated,
    ChatMessageEdited,
    ChatMessageRemoved,
    ChatMessageReplied,
    ChatRead,
    ChatTyping,
    CommentReplied,
    DonationProcessed,
    EffectApplied,
    EffectRemoved,
    FandomCuratorAssigned,
    FandomCuratorRevoked,
    FandomModeratorGranted,
    FandomModeratorRevoked,
    FandomRemovalRejected,
    FandomReviewed,
    FollowedPostCreated,
    ImportantPostCreated,
    PostClosed,
    PostDrafted,
    PostFandomChanged,
    PostImagesPurged,
    PostMultilingualDisabled,
    PostNsfwToggled,
    PostOpened,
    PostRelayPostCreated,
    PostRelayTurnAssigned,
    PostRelayTurnMissed,
    PostRelayTurnRejected,
    PostTagsChanged,
    ProfileDescriptionCleared,
    ProfileLinkRemoved,
    ProfileNameCleared,
    ProfileStatusCleared,
    PublicationBlocked,
    PublicationBlockedAfterReport,
    PublicationCommented,
    PublicationRated,
    PublicationReacted,
    PublicationRestored,
    PunishmentRemoved,
    RubricFandomChanged,
    RubricKarmaCoefChanged,
    RubricNameChanged,
    RubricOwnerAssigned,
    RubricOwnerTransferred,
    RubricRemoved,
    #[cfg_attr(feature = "serde", serde(untagged))]
    Unknown(i64),
}
