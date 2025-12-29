/// Account implementation and helper structs.
pub mod account;
/// Authentication implementation, errors and helper structs.
pub mod auth;
/// Common, shared models used across the API.
pub mod common;
/// Fandom implementations and helper structs.
pub mod fandom;
/// Publication implementations and helper structs.
pub mod publication;

/* --- Queries ---
 * [ ] AccountSecurityQuery
 * [ ] BindOAuthMutation
 * [ ] ChangeEmailMutation
 * [ ] ChangePasswordMutation
 * [ ] CreateSecurityIntentionMutation
 * [x] LoginEmailMutation
 * [ ] LoginOAuthMutation
 * [x] LoginRefreshMutation
 * [x] LogoutMutation
 * [x] MeQuery
 * [ ] OAuthUrlQuery
 * [ ] RegisterEmailMutation
 * [ ] ResendVerificationMutation
 * [ ] SavePlayIntegrityMutation
 * [ ] SendPasswordRecoveryMutation
 * [x] SetBirthdayMutation
 * [ ] TerminateSessionMutation
 *
 * --- Requests ---
 * [ ] RAccountSetSettings
 * [ ] RAccountsAchievementsRecount
 * [ ] RAccountsAddEmail
 * [ ] RAccountsAddGoogle
 * [ ] RAccountsAddNotificationsToken
 * [ ] RAccountsAdminBan
 * [ ] RAccountsAdminChangeName
 * [ ] RAccountsAdminEffectAdd
 * [ ] RAccountsAdminEffectRemove
 * [ ] RAccountsAdminPunishmentsRemove
 * [ ] RAccountsAdminRemoveDescription
 * [ ] RAccountsAdminRemoveLink
 * [ ] RAccountsAdminStatusRemove
 * [x] RAccountsBioSetAge
 * [x] RAccountsBioSetDescription
 * [ ] RAccountsBioSetLink
 * [x] RAccountsBioSetSex
 * [ ] RAccountsBlackListAdd
 * [ ] RAccountsBlackListCheck
 * [ ] RAccountsBlackListGetAll
 * [ ] RAccountsBlackListRemove
 * [ ] RAccountsChangeAvatar
 * [ ] RAccountsChangeName
 * [ ] RAccountsChangeNote
 * [ ] RAccountsChangeTitleImage
 * [ ] RAccountsClearReports
 * [ ] RAccountsFirebaseAdd
 * [ ] RAccountsFollowsChange
 * [ ] RAccountsFollowsGetAll
 * [x] RAccountsGet
 * [x] RAccountsGetAll
 * [x] RAccountsGetAllOnline
 * [ ] RAccountsGetIgnoredFandoms
 * [ ] RAccountsGetInfo
 * [x] RAccountsGetProfile
 * [ ] RAccountsGetStory
 * [ ] RAccountsKarmaInFandomsGetAll
 * [ ] RAccountsKarmaRecount
 * [ ] RAccountsLogin
 * [ ] RAccountsLoginSimple
 * [ ] RAccountsLogout
 * [ ] RAccountsNotificationsGetAll
 * [ ] RAccountsNotificationsView
 * [ ] RAccountsPrisonGetAll
 * [ ] RAccountsProtoadminAutorization
 * [ ] RAccountsPunishmentsGetAll
 * [ ] RAccountsPunishmentsGetInfo
 * [ ] RAccountsRatesGetAll
 * [ ] RAccountsRatingGet
 * [ ] RAccountsRemoveAvatar
 * [ ] RAccountsRemoveName
 * [ ] RAccountsRemoveTitleImage
 * [ ] RAccountsReport
 * [ ] RAccountsReportsGetAll
 * [ ] RAccountsReportsGetAllForAccount
 * [ ] RAccountsSetRecruiter
 * [x] RAccountsStatusSet
 * [ ] RAchievementsInfo
 * [ ] RAchievementsOnFinish
 * [ ] RAchievementsPack
 * [ ] RActivitiesGet
 * [ ] RActivitiesGetAllForAccount
 * [ ] RActivitiesGetAllNotForAccount
 * [ ] RActivitiesGetCounts
 * [ ] RActivitiesGetPosts
 * [ ] RActivitiesGetRelayRaceFullInfo
 * [ ] RActivitiesRelayGetMembers
 * [ ] RActivitiesRelayGetRejected
 * [ ] RActivitiesRelayRaceChange
 * [ ] RActivitiesRelayRaceCheckNextUser
 * [ ] RActivitiesRelayRaceCreate
 * [ ] RActivitiesRelayRaceMember
 * [ ] RActivitiesRelayRaceReject
 * [ ] RActivitiesRemove
 * [ ] RActivitiesSubscribe
 * [ ] RActivitiesSubscribeGet
 * [ ] RAdminVoteAccept
 * [ ] RAdminVoteCancel
 * [ ] RAdminVoteGet
 * [ ] RBookmarksAdd
 * [ ] RBookmarksGetAll
 * [ ] RBookmarksRemove
 * [ ] RBookmarksStatus
 * [ ] RChatChange
 * [ ] RChatCreate
 * [ ] RChatEnter
 * [ ] RChatGet
 * [ ] RChatGetForChange
 * [ ] RChatGetInfo
 * [ ] RChatGetSubscribers
 * [ ] RChatLeave
 * [ ] RChatMessageChange
 * [ ] RChatMessageCreate
 * [ ] RChatMessageGet
 * [ ] RChatMessageGetAll
 * [ ] RChatRead
 * [ ] RChatRemove
 * [ ] RChatSetBackgroundImage
 * [ ] RChatSubscribe
 * [ ] RChatTyping
 * [ ] RChatsFandomGetAll
 * [ ] RChatsGetAll
 * [ ] RCommentGet
 * [ ] RCommentsChange
 * [ ] RCommentsCreate
 * [ ] RCommentsGetAll
 * [ ] RCommentsWatchChange
 * [ ] RFandomsAccept
 * [ ] RFandomsAdminChangeCategory
 * [ ] RFandomsAdminChangeImage
 * [ ] RFandomsAdminChangeName
 * [ ] RFandomsAdminChangeParams
 * [ ] RFandomsAdminClose
 * [ ] RFandomsAdminMakeModerator
 * [ ] RFandomsAdminRemove
 * [ ] RFandomsAdminRemoveModerator
 * [ ] RFandomsAdminSetCof
 * [ ] RFandomsAdminViceroyAssign
 * [ ] RFandomsAdminViceroyRemove
 * [ ] RFandomsBlackListAdd
 * [ ] RFandomsBlackListContains
 * [ ] RFandomsBlackListRemove
 * [ ] RFandomsChange
 * [ ] RFandomsGet
 * [ ] RFandomsGetAll
 * [ ] RFandomsGetAllById
 * [ ] RFandomsGetAllModerated
 * [ ] RFandomsGetAllSubscribed
 * [ ] RFandomsGetAllViceroy
 * [ ] RFandomsGetBackground
 * [ ] RFandomsGetInfo
 * [ ] RFandomsGetPinedPost
 * [ ] RFandomsGetProfile
 * [ ] RFandomsGetSubscribtion
 * [ ] RFandomsModerationBlock
 * [ ] RFandomsModerationChangeImageBackground
 * [ ] RFandomsModerationChangeImageTitle
 * [ ] RFandomsModerationChatChange
 * [ ] RFandomsModerationChatCreate
 * [ ] RFandomsModerationChatRemove
 * [ ] RFandomsModerationDescriptionChange
 * [ ] RFandomsModerationForgive
 * [ ] RFandomsModerationGalleryAdd
 * [ ] RFandomsModerationGalleryRemove
 * [ ] RFandomsModerationGet
 * [ ] RFandomsModerationImportant
 * [ ] RFandomsModerationLinkAdd
 * [ ] RFandomsModerationLinkChange
 * [ ] RFandomsModerationLinkRemove
 * [ ] RFandomsModerationNames
 * [ ] RFandomsModerationToDrafts
 * [ ] RFandomsModerationsGetAll
 * [ ] RFandomsModeratorsGetAll
 * [ ] RFandomsPrisonGetAll
 * [ ] RFandomsRatingGet
 * [ ] RFandomsSubscribeChange
 * [ ] RFandomsSubscribersGetAll
 * [ ] RFandomsSuggest
 * [ ] RFandomsSuggestedGet
 * [ ] RFandomsSuggestedGetAll
 * [ ] RFandomsViceroyGet
 * [ ] RPostAdminRemoveMedia
 * [ ] RPostChangeFandom
 * [ ] RPostChangePage
 * [ ] RPostClose
 * [ ] RPostCloseModerator
 * [ ] RPostCloseNo
 * [ ] RPostCloseNoModerator
 * [ ] RPostDuplicateDraft
 * [ ] RPostFeedGetAll
 * [ ] RPostFeedGetAllSubscribe
 * [ ] RPostGet
 * [ ] RPostGetAllByRubric
 * [ ] RPostGetAllByTag
 * [ ] RPostGetDraft
 * [ ] RPostMakeMultilingual
 * [ ] RPostMakeMultilingualModeratorNot
 * [ ] RPostMakeMultilingualNot
 * [ ] RPostMenuInfoGet
 * [ ] RPostMovePage
 * [ ] RPostMoveRubric
 * [ ] RPostNotifyFollowers
 * [ ] RPostPagePollingGet
 * [ ] RPostPagePollingGetVotes
 * [ ] RPostPagePollingVote
 * [ ] RPostPendingGetAll
 * [ ] RPostPendingPublish
 * [ ] RPostPinAccount
 * [ ] RPostPinFandom
 * [ ] RPostPublication
 * [ ] RPostPutPage
 * [ ] RPostRatesGetAll
 * [ ] RPostRemove
 * [ ] RPostRemovePage
 * [ ] RPostSetNsfw
 * [ ] RPostSetNsfwModerator
 * [ ] RPostToDrafts
 * [ ] RProjectABParamsSet
 * [ ] RProjectDonatesCreateDraft
 * [ ] RProjectDonatesDraftsGetAll
 * [ ] RProjectDonatesGetAll
 * [ ] RProjectDonatesRatingsGetAll30
 * [ ] RProjectDonatesRatingsGetAllTotal
 * [ ] RProjectGetEffect
 * [ ] RProjectGetEvents
 * [ ] RProjectGetLoadingPictures
 * [ ] RProjectKeyGet
 * [ ] RProjectKeySet
 * [ ] RProjectMakeHelloPost
 * [ ] RProjectMiniGameAddScore
 * [ ] RProjectMiniGameGetinfo
 * [ ] RProjectStatistic
 * [ ] RProjectSupportAdd
 * [ ] RProjectSupportGetInfo
 * [ ] RProjectVersionGet
 * [ ] RPublicationsAdminClearReports
 * [ ] RPublicationsAdminRemove
 * [ ] RPublicationsAdminRestore
 * [ ] RPublicationsAdminRestoreDeepBlock
 * [ ] RPublicationsBlockGetAll
 * [ ] RPublicationsDraftsGetAll
 * [ ] RPublicationsGetAll
 * [ ] RPublicationsGetAllDeepBlocked
 * [ ] RPublicationsHistoryGetAll
 * [ ] RPublicationsKarmaAdd
 * [ ] RPublicationsOnShare
 * [ ] RPublicationsReactionAdd
 * [ ] RPublicationsReactionGetAccounts
 * [ ] RPublicationsReactionRemove
 * [ ] RPublicationsRemove
 * [ ] RPublicationsReport
 * [ ] RPublicationsReportedGetAll
 * [ ] RPublicationsReportsGetAll
 * [ ] RQuestsAddPart
 * [ ] RQuestsChangePart
 * [ ] RQuestsGet
 * [ ] RQuestsGetDrafts
 * [ ] RQuestsGetLatest
 * [ ] RQuestsGetParts
 * [ ] RQuestsModify
 * [ ] RQuestsNew
 * [ ] RQuestsPublish
 * [ ] RQuestsRemovePart
 * [ ] RQuestsReorderPart
 * [ ] RQuestsSaveState
 * [ ] RRubricGet
 * [ ] RRubricsChangeNotifications
 * [ ] RRubricsGetAll
 * [ ] RRubricsGetParams
 * [ ] RRubricsModerChangeName
 * [ ] RRubricsModerChangeOwner
 * [ ] RRubricsModerCreate
 * [ ] RRubricsModerRemove
 * [ ] RRubricsMoveFandom
 * [ ] RStickerCollectionAdd
 * [ ] RStickerCollectionCheck
 * [ ] RStickerCollectionRemove
 * [ ] RStickersAdd
 * [ ] RStickersGetAllByAccount
 * [ ] RStickersGetAllByPackId
 * [ ] RStickersGetAllFavorite
 * [ ] RStickersPackChange
 * [ ] RStickersPackChangeAvatar
 * [ ] RStickersPackCollectionAdd
 * [ ] RStickersPackCollectionCheck
 * [ ] RStickersPackCollectionRemove
 * [ ] RStickersPackCreate
 * [ ] RStickersPackRename
 * [ ] RStickersPacksGetAllByAccount
 * [ ] RStickersPacksGetInfo
 * [ ] RStickersSearch
 * [ ] RTagsChange
 * [ ] RTagsCreate
 * [ ] RTagsGet
 * [ ] RTagsGetAll
 * [ ] RTagsMove
 * [ ] RTagsMoveCategory
 * [ ] RTagsMoveTag
 * [ ] RTagsRemove
 * [ ] RTranslateChange
 * [ ] RTranslateConfirm
 * [ ] RTranslateGetMap
 * [ ] RTranslateHintChange
 * [ ] RTranslateHistoryGet
 * [ ] RTranslateModerationGet
 * [ ] RTranslateReject
 * [ ] RWikiArticleChangeLanguage
 * [ ] RWikiGet
 * [ ] RWikiGetPages
 * [ ] RWikiItemChange
 * [ ] RWikiItemCreate
 * [ ] RWikiItemGet
 * [ ] RWikiItemHistoryCancel
 * [ ] RWikiItemHistoryGet
 * [ ] RWikiItemHistoryRestore
 * [ ] RWikiItemMove
 * [ ] RWikiListGet
 * [ ] RWikiPageChange
 * [ ] RWikiPageMove
 * [ ] RWikiPagePut
 * [ ] RWikiPageRemove
 * [ ] RWikiRemove
 * [ ] RWikiReorder
 */

pub use account::{Account, Badge, Effect, Gender, Info as AccountInfo, Link};
pub use auth::{Auth, Me};
use chrono::{DateTime, Utc};
pub use common::{Category, ImageRef, Language};
pub use fandom::Fandom;
pub use publication::{Post, Publication, Reaction};
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}

pub(crate) fn serialize_level<S: Serializer>(
    value: &f32,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    // 12.34 -> 1234
    serializer.serialize_i64((value * 100.) as i64)
}

pub(crate) fn deserialize_level<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<f32, D::Error> {
    // 1234 -> 12.34
    Ok(i64::deserialize(deserializer)? as f32 / 100.)
}

pub(crate) fn serialize_level_or_none<S: Serializer>(
    value: &Option<f32>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_i64(match value {
        None => 0,
        Some(value) => (value * 100.) as i64,
    })
}

pub(crate) fn deserialize_level_or_none<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<f32>, D::Error> {
    Ok(match i64::deserialize(deserializer)? {
        0 => None,
        value => Some(value as f32 / 100.),
    })
}

pub(crate) fn serialize_timestamp_millis<S: Serializer>(
    value: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_i64(value.timestamp_millis())
}

pub(crate) fn deserialize_timestamp_millis<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error> {
    DateTime::from_timestamp_millis(i64::deserialize(deserializer)?)
        .ok_or_else(|| D::Error::custom("timestamp is out of range"))
}

pub(crate) fn serialize_timestamp_millis_or_none<S: Serializer>(
    value: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_i64(match value {
        None => 0,
        Some(value) => value.timestamp_millis(),
    })
}

pub(crate) fn deserialize_timestamp_millis_or_none<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error> {
    // Sometimes, the server returns 0 to indicate that something has never happened
    Ok(match i64::deserialize(deserializer)? {
        0 => None,
        timestamp => Some(
            DateTime::from_timestamp_millis(timestamp)
                .ok_or_else(|| D::Error::custom("timestamp is out of range"))?,
        ),
    })
}

pub(crate) fn serialize_string_or_none<S: Serializer>(
    value: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(match value {
        None => "",
        Some(value) => value,
    })
}

pub(crate) fn deserialize_string_or_none<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<String>, D::Error> {
    let value = String::deserialize(deserializer)?;
    Ok(match value.as_str() {
        "" => None,
        _ => Some(value),
    })
}

pub(crate) fn serialize_i64_or_none<S: Serializer>(
    value: &Option<i64>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_i64(match value {
        None => 0,
        Some(value) => *value,
    })
}

pub(crate) fn deserialize_i64_or_none<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<i64>, D::Error> {
    let value = i64::deserialize(deserializer)?;
    Ok(match value {
        0 => None,
        _ => Some(value),
    })
}
