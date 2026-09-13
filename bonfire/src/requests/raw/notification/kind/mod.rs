mod account;
mod any;
mod chat;
mod fandom;
mod other;
mod post;
mod profile;
mod publication;
mod rubric;

use std::result::Result as StdResult;

pub(crate) use account::{
    RawEffectApplied, RawEffectRemoved, RawFandomUnbanned as RawAccountFandomUnbanned,
    RawMentioned as RawAccountMentioned, RawPunished as RawAccountPunished, RawPunishmentRemoved,
};
pub(crate) use any::AnyRawNotification;
pub(crate) use chat::{
    RawMessageCreated as RawChatMessageCreated, RawMessageEdited as RawChatMessageEdited,
    RawMessageRemoved as RawChatMessageRemoved, RawMessageReplied as RawChatMessageReplied,
    RawRead as RawChatRead, RawTyping as RawChatTyping,
};
pub(crate) use fandom::{
    RawCuratorAssigned as RawFandomCuratorAssigned, RawCuratorRevoked as RawFandomCuratorRevoked,
    RawModeratorSet as RawFandomModeratorSet, RawRemovalRejected as RawFandomRemovalRejected,
    RawReviewed as RawFandomReviewed,
};
pub(crate) use other::{RawAdminActionRejected, RawDonationProcessed};
pub(crate) use post::{
    RawFandomChanged as RawPostFandomChanged, RawFollowedPostCreated,
    RawImagesPurged as RawPostImagesPurged, RawImportantPostCreated,
    RawMultilingualDisabled as RawPostMultilingualDisabled, RawNsfwToggled as RawPostNsfwToggled,
    RawRelayPostCreated as RawPostRelayPostCreated,
    RawRelayTurnAssigned as RawPostRelayTurnAssigned, RawRelayTurnMissed as RawPostRelayTurnMissed,
    RawRelayTurnRejected as RawPostRelayTurnRejected, RawTagsChanged as RawPostTagsChanged,
    RawVisibilityChanged as RawPostVisibilityChanged,
};
pub(crate) use profile::{RawAchievementUnlocked, RawFieldSet as RawProfileFieldSet};
pub(crate) use publication::{
    RawBlockRejected as RawPublicationBlockRejected, RawBlocked as RawPublicationBlocked,
    RawBlockedAfterReport as RawPublicationBlockedAfterReport, RawCommentReplied,
    RawCommented as RawPublicationCommented, RawDrafted as RawPublicationDrafted,
    RawRated as RawPublicationRated, RawReacted as RawPublicationReacted,
    RawRestored as RawPublicationRestored,
};
pub(crate) use rubric::{
    RawFandomChanged as RawRubricFandomChanged, RawKarmaCoefChanged as RawRubricKarmaCoefChanged,
    RawNameChanged as RawRubricNameChanged, RawOwnerAssigned as RawRubricOwnerAssigned,
    RawOwnerTransferred as RawRubricOwnerTransferred, RawRemoved as RawRubricRemoved,
};
use serde::{Deserialize, Deserializer};

#[derive(Debug)]
pub(crate) enum RawKind {
    AccountFandomUnbanned,
    AccountFollowed,
    AccountMentioned,
    AccountPunished,
    AccountTargetAdminActionRejected,
    AccountUnfollowed,
    AchievementUnlocked,
    AdminActionRejected,
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
    PublicationBlockRejected,
    PublicationBlocked,
    PublicationBlockedAfterReport,
    PublicationCommented,
    PublicationDrafted,
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
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawKind::PublicationRated,
            2 => RawKind::PublicationCommented,
            3 => RawKind::CommentReplied,
            4 => RawKind::AccountFollowed,
            7 => RawKind::AchievementUnlocked,
            8 => RawKind::ChatMessageCreated,
            9 => RawKind::ChatMessageReplied,
            10 => RawKind::FollowedPostCreated,
            11 => RawKind::ChatMessageEdited,
            12 => RawKind::PublicationBlocked,
            13 => RawKind::ChatMessageRemoved,
            14 => RawKind::ChatTyping,
            15 => RawKind::FandomReviewed,
            19 => RawKind::ImportantPostCreated,
            20 => RawKind::PublicationDrafted,
            22 => RawKind::PostTagsChanged,
            23 => RawKind::AccountFandomUnbanned,
            24 => RawKind::ChatRead,
            25 => RawKind::AccountPunished,
            26 => RawKind::FandomModeratorGranted,
            27 => RawKind::FandomModeratorRevoked,
            28 => RawKind::PunishmentRemoved,
            29 => RawKind::PublicationRestored,
            30 => RawKind::PublicationBlockRejected,
            31 => RawKind::ProfileStatusCleared,
            32 => RawKind::ProfileDescriptionCleared,
            33 => RawKind::ProfileNameCleared,
            34 => RawKind::ProfileLinkRemoved,
            35 => RawKind::PostFandomChanged,
            36 => RawKind::PublicationBlockedAfterReport,
            37 => RawKind::AccountMentioned,
            39 => RawKind::PostMultilingualDisabled,
            41 => RawKind::PostClosed,
            42 => RawKind::PostOpened,
            43 => RawKind::RubricNameChanged,
            44 => RawKind::RubricOwnerTransferred,
            45 => RawKind::RubricOwnerAssigned,
            46 => RawKind::RubricRemoved,
            47 => RawKind::RubricKarmaCoefChanged,
            48 => RawKind::PublicationReacted,
            49 => RawKind::PostRelayTurnAssigned,
            50 => RawKind::PostRelayTurnMissed,
            51 => RawKind::PostRelayPostCreated,
            52 => RawKind::PostRelayTurnRejected,
            53 => RawKind::FandomCuratorAssigned,
            54 => RawKind::FandomCuratorRevoked,
            56 => RawKind::DonationProcessed,
            57 => RawKind::EffectApplied,
            58 => RawKind::EffectRemoved,
            61 => RawKind::PostImagesPurged,
            62 => RawKind::FandomRemovalRejected,
            63 => RawKind::AdminActionRejected,
            64 => RawKind::AccountTargetAdminActionRejected,
            65 => RawKind::RubricFandomChanged,
            66 => RawKind::AccountUnfollowed,
            67 => RawKind::PostNsfwToggled,
            other => RawKind::Unknown(other),
        })
    }
}
