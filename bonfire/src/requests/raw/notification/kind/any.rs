use serde_json::Value;

use super::*;
use crate::models::AnyNotification;
use crate::requests::raw::RawAccountRef;
use crate::{Error, Result};

pub(crate) enum AnyRawNotification {
    AccountFandomUnbanned(RawAccountFandomUnbanned),
    AccountFollowed(RawAccountRef),
    AccountMentioned(RawAccountMentioned),
    AccountPunished(RawAccountPunished),
    AccountTargetAdminActionRejected(RawAdminActionRejected),
    AccountUnfollowed(RawAccountRef),
    AchievementUnlocked(RawAchievementUnlocked),
    AdminActionRejected(RawAdminActionRejected),
    ChatMessageCreated(RawChatMessageCreated),
    ChatMessageEdited(RawChatMessageEdited),
    ChatMessageRemoved(RawChatMessageRemoved),
    ChatMessageReplied(RawChatMessageReplied),
    ChatRead(RawChatRead),
    ChatTyping(RawChatTyping),
    CommentReplied(RawCommentReplied),
    DonationProcessed(RawDonationProcessed),
    EffectApplied(RawEffectApplied),
    EffectRemoved(RawEffectRemoved),
    FandomCuratorAssigned(RawFandomCuratorAssigned),
    FandomCuratorRevoked(RawFandomCuratorRevoked),
    FandomModeratorGranted(RawFandomModeratorSet),
    FandomModeratorRevoked(RawFandomModeratorSet),
    FandomRemovalRejected(RawFandomRemovalRejected),
    FandomReviewed(RawFandomReviewed),
    FollowedPostCreated(RawFollowedPostCreated),
    ImportantPostCreated(RawImportantPostCreated),
    PostClosed(RawPostVisibilityChanged),
    PostFandomChanged(RawPostFandomChanged),
    PostImagesPurged(RawPostImagesPurged),
    PostMultilingualDisabled(RawPostMultilingualDisabled),
    PostNsfwToggled(RawPostNsfwToggled),
    PostOpened(RawPostVisibilityChanged),
    PostRelayPostCreated(RawPostRelayPostCreated),
    PostRelayTurnAssigned(RawPostRelayTurnAssigned),
    PostRelayTurnMissed(RawPostRelayTurnMissed),
    PostRelayTurnRejected(RawPostRelayTurnRejected),
    PostTagsChanged(RawPostTagsChanged),
    ProfileDescriptionCleared(RawProfileFieldSet),
    ProfileLinkRemoved(RawProfileFieldSet),
    ProfileNameCleared(RawProfileFieldSet),
    ProfileStatusCleared(RawProfileFieldSet),
    PublicationBlockRejected(RawPublicationBlockRejected),
    PublicationBlocked(RawPublicationBlocked),
    PublicationBlockedAfterReport(RawPublicationBlockedAfterReport),
    PublicationCommented(RawPublicationCommented),
    PublicationDrafted(RawPublicationDrafted),
    PublicationRated(RawPublicationRated),
    PublicationReacted(RawPublicationReacted),
    PublicationRestored(RawPublicationRestored),
    PunishmentRemoved(RawPunishmentRemoved),
    RubricFandomChanged(RawRubricFandomChanged),
    RubricKarmaCoefChanged(RawRubricKarmaCoefChanged),
    RubricNameChanged(RawRubricNameChanged),
    RubricOwnerAssigned(RawRubricOwnerAssigned),
    RubricOwnerTransferred(RawRubricOwnerTransferred),
    RubricRemoved(RawRubricRemoved),
    Unknown(i64),
}
impl AnyRawNotification {
    pub(crate) fn new(data: Value, kind: RawKind) -> Result<Self> {
        // TODO: replace this and TryFrom with macros (derive?)
        Ok(match kind {
            RawKind::AccountFandomUnbanned => {
                AnyRawNotification::AccountFandomUnbanned(serde_json::from_value(data)?)
            }
            RawKind::AccountFollowed => {
                AnyRawNotification::AccountFollowed(serde_json::from_value(data)?)
            }
            RawKind::AccountMentioned => {
                AnyRawNotification::AccountMentioned(serde_json::from_value(data)?)
            }
            RawKind::AccountPunished => {
                AnyRawNotification::AccountPunished(serde_json::from_value(data)?)
            }
            RawKind::AccountTargetAdminActionRejected => {
                AnyRawNotification::AccountTargetAdminActionRejected(serde_json::from_value(data)?)
            }
            RawKind::AccountUnfollowed => {
                AnyRawNotification::AccountUnfollowed(serde_json::from_value(data)?)
            }
            RawKind::AchievementUnlocked => {
                AnyRawNotification::AchievementUnlocked(serde_json::from_value(data)?)
            }
            RawKind::AdminActionRejected => {
                AnyRawNotification::AdminActionRejected(serde_json::from_value(data)?)
            }
            RawKind::ChatMessageCreated => {
                AnyRawNotification::ChatMessageCreated(serde_json::from_value(data)?)
            }
            RawKind::ChatMessageEdited => {
                AnyRawNotification::ChatMessageEdited(serde_json::from_value(data)?)
            }
            RawKind::ChatMessageRemoved => {
                AnyRawNotification::ChatMessageRemoved(serde_json::from_value(data)?)
            }
            RawKind::ChatMessageReplied => {
                AnyRawNotification::ChatMessageReplied(serde_json::from_value(data)?)
            }
            RawKind::ChatRead => AnyRawNotification::ChatRead(serde_json::from_value(data)?),
            RawKind::ChatTyping => AnyRawNotification::ChatTyping(serde_json::from_value(data)?),
            RawKind::CommentReplied => {
                AnyRawNotification::CommentReplied(serde_json::from_value(data)?)
            }
            RawKind::DonationProcessed => {
                AnyRawNotification::DonationProcessed(serde_json::from_value(data)?)
            }
            RawKind::EffectApplied => {
                AnyRawNotification::EffectApplied(serde_json::from_value(data)?)
            }
            RawKind::EffectRemoved => {
                AnyRawNotification::EffectRemoved(serde_json::from_value(data)?)
            }
            RawKind::FandomCuratorAssigned => {
                AnyRawNotification::FandomCuratorAssigned(serde_json::from_value(data)?)
            }
            RawKind::FandomCuratorRevoked => {
                AnyRawNotification::FandomCuratorRevoked(serde_json::from_value(data)?)
            }
            RawKind::FandomModeratorGranted => {
                AnyRawNotification::FandomModeratorGranted(serde_json::from_value(data)?)
            }
            RawKind::FandomModeratorRevoked => {
                AnyRawNotification::FandomModeratorRevoked(serde_json::from_value(data)?)
            }
            RawKind::FandomRemovalRejected => {
                AnyRawNotification::FandomRemovalRejected(serde_json::from_value(data)?)
            }
            RawKind::FandomReviewed => {
                AnyRawNotification::FandomReviewed(serde_json::from_value(data)?)
            }
            RawKind::FollowedPostCreated => {
                AnyRawNotification::FollowedPostCreated(serde_json::from_value(data)?)
            }
            RawKind::ImportantPostCreated => {
                AnyRawNotification::ImportantPostCreated(serde_json::from_value(data)?)
            }
            RawKind::PostClosed => AnyRawNotification::PostClosed(serde_json::from_value(data)?),
            RawKind::PostFandomChanged => {
                AnyRawNotification::PostFandomChanged(serde_json::from_value(data)?)
            }
            RawKind::PostImagesPurged => {
                AnyRawNotification::PostImagesPurged(serde_json::from_value(data)?)
            }
            RawKind::PostMultilingualDisabled => {
                AnyRawNotification::PostMultilingualDisabled(serde_json::from_value(data)?)
            }
            RawKind::PostNsfwToggled => {
                AnyRawNotification::PostNsfwToggled(serde_json::from_value(data)?)
            }
            RawKind::PostOpened => AnyRawNotification::PostOpened(serde_json::from_value(data)?),
            RawKind::PostRelayPostCreated => {
                AnyRawNotification::PostRelayPostCreated(serde_json::from_value(data)?)
            }
            RawKind::PostRelayTurnAssigned => {
                AnyRawNotification::PostRelayTurnAssigned(serde_json::from_value(data)?)
            }
            RawKind::PostRelayTurnMissed => {
                AnyRawNotification::PostRelayTurnMissed(serde_json::from_value(data)?)
            }
            RawKind::PostRelayTurnRejected => {
                AnyRawNotification::PostRelayTurnRejected(serde_json::from_value(data)?)
            }
            RawKind::PostTagsChanged => {
                AnyRawNotification::PostTagsChanged(serde_json::from_value(data)?)
            }
            RawKind::ProfileDescriptionCleared => {
                AnyRawNotification::ProfileDescriptionCleared(serde_json::from_value(data)?)
            }
            RawKind::ProfileLinkRemoved => {
                AnyRawNotification::ProfileLinkRemoved(serde_json::from_value(data)?)
            }
            RawKind::ProfileNameCleared => {
                AnyRawNotification::ProfileNameCleared(serde_json::from_value(data)?)
            }
            RawKind::ProfileStatusCleared => {
                AnyRawNotification::ProfileStatusCleared(serde_json::from_value(data)?)
            }
            RawKind::PublicationBlockRejected => {
                AnyRawNotification::PublicationBlockRejected(serde_json::from_value(data)?)
            }
            RawKind::PublicationBlocked => {
                AnyRawNotification::PublicationBlocked(serde_json::from_value(data)?)
            }
            RawKind::PublicationBlockedAfterReport => {
                AnyRawNotification::PublicationBlockedAfterReport(serde_json::from_value(data)?)
            }
            RawKind::PublicationCommented => {
                AnyRawNotification::PublicationCommented(serde_json::from_value(data)?)
            }
            RawKind::PublicationDrafted => {
                AnyRawNotification::PublicationDrafted(serde_json::from_value(data)?)
            }
            RawKind::PublicationRated => {
                AnyRawNotification::PublicationRated(serde_json::from_value(data)?)
            }
            RawKind::PublicationReacted => {
                AnyRawNotification::PublicationReacted(serde_json::from_value(data)?)
            }
            RawKind::PublicationRestored => {
                AnyRawNotification::PublicationRestored(serde_json::from_value(data)?)
            }
            RawKind::PunishmentRemoved => {
                AnyRawNotification::PunishmentRemoved(serde_json::from_value(data)?)
            }
            RawKind::RubricFandomChanged => {
                AnyRawNotification::RubricFandomChanged(serde_json::from_value(data)?)
            }
            RawKind::RubricKarmaCoefChanged => {
                AnyRawNotification::RubricKarmaCoefChanged(serde_json::from_value(data)?)
            }
            RawKind::RubricNameChanged => {
                AnyRawNotification::RubricNameChanged(serde_json::from_value(data)?)
            }
            RawKind::RubricOwnerAssigned => {
                AnyRawNotification::RubricOwnerAssigned(serde_json::from_value(data)?)
            }
            RawKind::RubricOwnerTransferred => {
                AnyRawNotification::RubricOwnerTransferred(serde_json::from_value(data)?)
            }
            RawKind::RubricRemoved => {
                AnyRawNotification::RubricRemoved(serde_json::from_value(data)?)
            }
            RawKind::Unknown(kind) => AnyRawNotification::Unknown(kind),
        })
    }
}

impl TryFrom<AnyRawNotification> for AnyNotification {
    type Error = Error;

    fn try_from(value: AnyRawNotification) -> Result<Self> {
        Ok(match value {
            AnyRawNotification::AccountFandomUnbanned(notification) => {
                AnyNotification::AccountFandomUnbanned(notification.try_into()?)
            }
            AnyRawNotification::AccountFollowed(notification) => {
                AnyNotification::AccountFollowed(notification.try_into()?)
            }
            AnyRawNotification::AccountMentioned(notification) => {
                AnyNotification::AccountMentioned(notification.try_into()?)
            }
            AnyRawNotification::AccountPunished(notification) => {
                AnyNotification::AccountPunished(notification.try_into()?)
            }
            AnyRawNotification::AccountTargetAdminActionRejected(notification) => {
                AnyNotification::AccountTargetAdminActionRejected(notification.try_into()?)
            }
            AnyRawNotification::AccountUnfollowed(notification) => {
                AnyNotification::AccountUnfollowed(notification.try_into()?)
            }
            AnyRawNotification::AchievementUnlocked(notification) => {
                AnyNotification::AchievementUnlocked(notification.into())
            }
            AnyRawNotification::AdminActionRejected(notification) => {
                AnyNotification::AdminActionRejected(notification.try_into()?)
            }
            AnyRawNotification::ChatMessageCreated(notification) => {
                AnyNotification::ChatMessageCreated(notification.try_into()?)
            }
            AnyRawNotification::ChatMessageEdited(notification) => {
                AnyNotification::ChatMessageEdited(notification.into())
            }
            AnyRawNotification::ChatMessageRemoved(notification) => {
                AnyNotification::ChatMessageRemoved(notification.into())
            }
            AnyRawNotification::ChatMessageReplied(notification) => {
                AnyNotification::ChatMessageReplied(notification.try_into()?)
            }
            AnyRawNotification::ChatRead(notification) => {
                AnyNotification::ChatRead(notification.try_into()?)
            }
            AnyRawNotification::ChatTyping(notification) => {
                AnyNotification::ChatTyping(notification.try_into()?)
            }
            AnyRawNotification::CommentReplied(notification) => {
                AnyNotification::CommentReplied(notification.try_into()?)
            }
            AnyRawNotification::DonationProcessed(notification) => {
                AnyNotification::DonationProcessed(notification.into())
            }
            AnyRawNotification::EffectApplied(notification) => {
                AnyNotification::EffectApplied(notification.try_into()?)
            }
            AnyRawNotification::EffectRemoved(notification) => {
                AnyNotification::EffectRemoved(notification.try_into()?)
            }
            AnyRawNotification::FandomCuratorAssigned(notification) => {
                AnyNotification::FandomCuratorAssigned(notification.try_into()?)
            }
            AnyRawNotification::FandomCuratorRevoked(notification) => {
                AnyNotification::FandomCuratorRevoked(notification.try_into()?)
            }
            AnyRawNotification::FandomModeratorGranted(notification) => {
                AnyNotification::FandomModeratorGranted(notification.try_into()?)
            }
            AnyRawNotification::FandomModeratorRevoked(notification) => {
                AnyNotification::FandomModeratorRevoked(notification.try_into()?)
            }
            AnyRawNotification::FandomRemovalRejected(notification) => {
                AnyNotification::FandomRemovalRejected(notification.try_into()?)
            }
            AnyRawNotification::FandomReviewed(notification) => {
                AnyNotification::FandomReviewed(notification.into())
            }
            AnyRawNotification::FollowedPostCreated(notification) => {
                AnyNotification::FollowedPostCreated(notification.try_into()?)
            }
            AnyRawNotification::ImportantPostCreated(notification) => {
                AnyNotification::ImportantPostCreated(notification.try_into()?)
            }
            AnyRawNotification::PostClosed(notification) => {
                AnyNotification::PostClosed(notification.try_into()?)
            }
            AnyRawNotification::PostFandomChanged(notification) => {
                AnyNotification::PostFandomChanged(notification.try_into()?)
            }
            AnyRawNotification::PostImagesPurged(notification) => {
                AnyNotification::PostImagesPurged(notification.try_into()?)
            }
            AnyRawNotification::PostMultilingualDisabled(notification) => {
                AnyNotification::PostMultilingualDisabled(notification.try_into()?)
            }
            AnyRawNotification::PostNsfwToggled(notification) => {
                AnyNotification::PostNsfwToggled(notification.try_into()?)
            }
            AnyRawNotification::PostOpened(notification) => {
                AnyNotification::PostOpened(notification.try_into()?)
            }
            AnyRawNotification::PostRelayPostCreated(notification) => {
                AnyNotification::PostRelayPostCreated(notification.try_into()?)
            }
            AnyRawNotification::PostRelayTurnAssigned(notification) => {
                AnyNotification::PostRelayTurnAssigned(notification.try_into()?)
            }
            AnyRawNotification::PostRelayTurnMissed(notification) => {
                AnyNotification::PostRelayTurnMissed(notification.try_into()?)
            }
            AnyRawNotification::PostRelayTurnRejected(notification) => {
                AnyNotification::PostRelayTurnRejected(notification.try_into()?)
            }
            AnyRawNotification::PostTagsChanged(notification) => {
                AnyNotification::PostTagsChanged(notification.try_into()?)
            }
            AnyRawNotification::ProfileDescriptionCleared(notification) => {
                AnyNotification::ProfileDescriptionCleared(notification.try_into()?)
            }
            AnyRawNotification::ProfileLinkRemoved(notification) => {
                AnyNotification::ProfileLinkRemoved(notification.try_into()?)
            }
            AnyRawNotification::ProfileNameCleared(notification) => {
                AnyNotification::ProfileNameCleared(notification.try_into()?)
            }
            AnyRawNotification::ProfileStatusCleared(notification) => {
                AnyNotification::ProfileStatusCleared(notification.try_into()?)
            }
            AnyRawNotification::PublicationBlockRejected(notification) => {
                AnyNotification::PublicationBlockRejected(notification.try_into()?)
            }
            AnyRawNotification::PublicationBlocked(notification) => {
                AnyNotification::PublicationBlocked(notification.try_into()?)
            }
            AnyRawNotification::PublicationBlockedAfterReport(notification) => {
                AnyNotification::PublicationBlockedAfterReport(notification.try_into()?)
            }
            AnyRawNotification::PublicationCommented(notification) => {
                AnyNotification::PublicationCommented(notification.try_into()?)
            }
            AnyRawNotification::PublicationDrafted(notification) => {
                AnyNotification::PublicationDrafted(notification.try_into()?)
            }
            AnyRawNotification::PublicationRated(notification) => {
                AnyNotification::PublicationRated(notification.try_into()?)
            }
            AnyRawNotification::PublicationReacted(notification) => {
                AnyNotification::PublicationReacted(notification.try_into()?)
            }
            AnyRawNotification::PublicationRestored(notification) => {
                AnyNotification::PublicationRestored(notification.into())
            }
            AnyRawNotification::PunishmentRemoved(notification) => {
                AnyNotification::PunishmentRemoved(notification.try_into()?)
            }
            AnyRawNotification::RubricFandomChanged(notification) => {
                AnyNotification::RubricFandomChanged(notification.try_into()?)
            }
            AnyRawNotification::RubricKarmaCoefChanged(notification) => {
                AnyNotification::RubricKarmaCoefChanged(notification.try_into()?)
            }
            AnyRawNotification::RubricNameChanged(notification) => {
                AnyNotification::RubricNameChanged(notification.try_into()?)
            }
            AnyRawNotification::RubricOwnerAssigned(notification) => {
                AnyNotification::RubricOwnerAssigned(notification.try_into()?)
            }
            AnyRawNotification::RubricOwnerTransferred(notification) => {
                AnyNotification::RubricOwnerTransferred(notification.try_into()?)
            }
            AnyRawNotification::RubricRemoved(notification) => {
                AnyNotification::RubricRemoved(notification.try_into()?)
            }
            AnyRawNotification::Unknown(kind) => AnyNotification::Unknown(kind),
        })
    }
}
