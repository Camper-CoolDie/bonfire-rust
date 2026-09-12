#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::*;
use crate::models::{AccountRef, ChatTag, Effect};

#[non_exhaustive]
#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum AnyNotification {
    AccountFandomUnbanned(AccountFandomUnbanned),
    AccountFollowed(AccountRef),
    AccountMentioned(AccountMentioned),
    AccountPunished(AccountPunished),
    AccountTargetAdminActionRejected(AdminActionRejected),
    AccountUnfollowed(AccountRef),
    AchievementUnlocked(AchievementUnlocked),
    AdminActionRejected(AdminActionRejected),
    ChatMessageCreated(ChatMessageCreated),
    ChatMessageEdited(ChatMessageEdited),
    ChatMessageRemoved(ChatMessageRemoved),
    ChatMessageReplied(ChatMessageReplied),
    ChatRead(ChatTag),
    ChatTyping(ChatTyping),
    CommentReplied(CommentReplied),
    DonationProcessed(DonationProcessed),
    EffectApplied(Effect),
    EffectRemoved(EffectRemoved),
    FandomCuratorAssigned(FandomCuratorAssigned),
    FandomCuratorRevoked(FandomCuratorRevoked),
    FandomModeratorGranted(FandomModeratorSet),
    FandomModeratorRevoked(FandomModeratorSet),
    FandomRemovalRejected(FandomRemovalRejected),
    FandomReviewed(FandomReviewed),
    FollowedPostCreated(FollowedPostCreated),
    ImportantPostCreated(ImportantPostCreated),
    PostClosed(PostVisibilityChanged),
    PostFandomChanged(PostFandomChanged),
    PostImagesPurged(PostImagesPurged),
    PostMultilingualDisabled(PostMultilingualDisabled),
    PostNsfwToggled(PostNsfwToggled),
    PostOpened(PostVisibilityChanged),
    PostRelayPostCreated(PostRelayPostCreated),
    PostRelayTurnAssigned(PostRelayTurnAssigned),
    PostRelayTurnMissed(PostRelayTurnMissed),
    PostRelayTurnRejected(PostRelayTurnRejected),
    PostTagsChanged(PostTagsChanged),
    ProfileDescriptionCleared(ProfileFieldSet),
    ProfileLinkRemoved(ProfileFieldSet),
    ProfileNameCleared(ProfileFieldSet),
    ProfileStatusCleared(ProfileFieldSet),
    PublicationBlockRejected(PublicationBlockRejected),
    PublicationBlocked(PublicationBlocked),
    PublicationBlockedAfterReport(PublicationBlocked),
    PublicationCommented(PublicationCommented),
    PublicationDrafted(PublicationDrafted),
    PublicationRated(PublicationRated),
    PublicationReacted(PublicationReacted),
    PublicationRestored(PublicationRestored),
    PunishmentRemoved(PunishmentRemoved),
    RubricFandomChanged(RubricFandomChanged),
    RubricKarmaCoefChanged(RubricKarmaCoefChanged),
    RubricNameChanged(RubricNameChanged),
    RubricOwnerAssigned(RubricOwnerAssigned),
    RubricOwnerTransferred(RubricOwnerTransferred),
    RubricRemoved(RubricRemoved),
    #[cfg_attr(feature = "serde", serde(untagged))]
    Unknown(i64),
}
