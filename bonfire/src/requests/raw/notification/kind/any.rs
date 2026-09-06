use serde::Deserialize;

use crate::models::AnyNotification;
use crate::requests::raw::account::RawEffectKind;
use crate::requests::raw::chat::RawKind as RawChatKind;
use crate::requests::raw::publication::{RawKind as RawPublicationKind, RawPostItemKind};
use crate::requests::raw::{
    RawAccount, RawChatMessage, RawChatTag, RawEffect, RawGender, RawLanguage, RawPublication,
};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "J_N_TYPE")]
pub(crate) enum AnyRawNotification {
    #[serde(rename = "1")]
    PublicationRated {
        #[serde(rename = "J_KARMA_COUNT")]
        amount: f64,
        #[serde(rename = "J_ACCOUNT_ID")]
        account_id: u64,
        #[serde(rename = "J_ACCOUNT_NAME")]
        account_name: String,
        #[serde(rename = "accountSex")]
        account_gender: RawGender,
        #[serde(rename = "J_UNIT_ID")]
        publication_id: u64,
        #[serde(rename = "J_UNIT_TYPE")]
        publication_kind: RawPublicationKind,
        #[serde(rename = "maskText")]
        post_title_text: String,
        #[serde(rename = "maskPageType")]
        post_title_item_kind: RawPostItemKind,
        #[serde(rename = "J_PARENT_UNIT_ID")]
        parent_id: u64,
        #[serde(rename = "J_PARENT_UNIT_TYPE")]
        parent_kind: RawPublicationKind,
    },
    #[serde(rename = "2")]
    PublicationCommented {
        #[serde(rename = "J_COMMENT_ID")]
        id: u64,
        #[serde(rename = "J_ACCOUNT_ID")]
        author_id: u64,
        #[serde(rename = "J_ACCOUNT_NAME")]
        author_name: String,
        #[serde(rename = "accountSex")]
        author_gender: RawGender,
        fandom_name: String,
        #[serde(rename = "J_UNIT_ID")]
        parent_id: u64,
        #[serde(rename = "J_PARENT_UNIT_TYPE")]
        parent_kind: RawPublicationKind,
        #[serde(rename = "unitCreatorId")]
        parent_author_id: u64,
        #[serde(rename = "maskText")]
        parent_post_title_text: String,
        #[serde(rename = "maskPageType")]
        parent_post_title_item_kind: RawPostItemKind,
        #[serde(rename = "commentText")]
        text: Option<String>,
    },
    #[serde(rename = "3")]
    CommentReplied {
        #[serde(rename = "J_COMMENT_ID")]
        id: u64,
        #[serde(rename = "J_ACCOUNT_ID")]
        author_id: u64,
        #[serde(rename = "J_ACCOUNT_NAME")]
        author_name: String,
        #[serde(rename = "accountSex")]
        author_gender: RawGender,
        #[serde(rename = "J_UNIT_ID")]
        parent_id: u64,
        #[serde(rename = "J_PARENT_UNIT_TYPE")]
        parent_kind: RawPublicationKind,
        #[serde(rename = "commentText")]
        text: Option<String>,
    },
    #[serde(rename = "4")]
    AccountFollowed {
        #[serde(rename = "J_ACCOUNT_ID")]
        account_id: u64,
        #[serde(rename = "J_ACCOUNT_NAME")]
        account_name: String,
        #[serde(rename = "accountSex")]
        account_gender: RawGender,
    },
    #[serde(rename = "7")]
    AchievementUnlocked,
    #[serde(rename = "8")]
    ChatMessageCreated {
        #[serde(rename = "unitChatMessage")]
        message: RawPublication<RawChatMessage>,
        #[serde(rename = "tag")]
        chat_tag: RawChatTag,
        #[serde(rename = "subscribed")]
        is_subscribed: bool,
    },
    #[serde(rename = "9")]
    ChatMessageReplied {
        #[serde(rename = "unitChatMessage")]
        reply: RawPublication<RawChatMessage>,
        #[serde(rename = "tag")]
        chat_tag: RawChatTag,
        #[serde(rename = "subscribed")]
        is_subscribed: bool,
    },
    #[serde(rename = "10")]
    FollowedPostCreated {
        #[serde(rename = "J_UNIT_ID")]
        id: u64,
        #[serde(rename = "J_ACCOUNT_ID")]
        author_id: u64,
        #[serde(rename = "J_ACCOUNT_NAME")]
        author_name: String,
        #[serde(rename = "accountSex")]
        author_gender: RawGender,
    },
    #[serde(rename = "11")]
    ChatMessageEdited {
        #[serde(rename = "J_UNIT_ID")]
        id: u64,
        #[serde(rename = "J_TEXT")]
        new_text: String,
    },
    #[serde(rename = "12")]
    PublicationBlocked {
        #[serde(rename = "blockUnitType")]
        publication_kind: RawPublicationKind,
        #[serde(rename = "J_MODERATION_ID")]
        moderation_id: u64,
        #[serde(rename = "J_BLOCK_LAST")]
        with_last_publications: bool,
        #[serde(rename = "J_BLOCK_ACCOUNT_DATE")]
        banned_until: i64,
        #[serde(rename = "J_COMMENT")]
        reason: String,
    },
    #[serde(rename = "13")]
    ChatMessageRemoved {
        #[serde(rename = "J_UNIT_ID")]
        id: u64,
    },
    #[serde(rename = "14")]
    ChatTyping {
        account_id: u64,
        account_name: String,
        chat_tag: RawChatTag,
    },
    #[serde(rename = "15")]
    FandomReviewed {
        #[serde(rename = "accepted")]
        is_accepted: bool,
        fandom_id: u64,
        fandom_name: String,
        admin_name: String,
        #[serde(rename = "comment")]
        note: String,
    },
    #[serde(rename = "19")]
    ImportantPostCreated {
        #[serde(rename = "unitId")]
        id: u64,
        fandom_id: u64,
        #[serde(rename = "fandomLanguageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        #[serde(rename = "moderatorAccountId")]
        importance_moderator_id: u64,
        #[serde(rename = "comment")]
        importance_reason: String,
    },
    #[serde(rename = "20")]
    PostDrafted {
        #[serde(rename = "maskText")]
        title_text: String,
        #[serde(rename = "maskPageType")]
        title_item_kind: RawPostItemKind,
        moderation_id: u64,
        moderator_name: String,
        #[serde(rename = "moderatorSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "22")]
    PostTagsChanged {
        moderation_id: u64,
        moderator_id: u64,
        moderator_name: String,
        #[serde(rename = "moderatorSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "23")]
    AccountFandomUnbanned {
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        moderator_id: u64,
        moderator_name: String,
        #[serde(rename = "moderatorSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "24")]
    ChatRead {
        #[serde(rename = "tag")]
        chat_tag: RawChatTag,
    },
    #[serde(rename = "25")]
    AccountPunished {
        #[serde(rename = "J_BLOCK_ACCOUNT_DATE")]
        banned_until: i64,
        #[serde(rename = "J_COMMENT")]
        reason: String,
    },
    #[serde(rename = "26")]
    FandomModeratorGranted {
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "27")]
    FandomModeratorRevoked {
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "28")]
    PunishmentRemoved {
        #[serde(rename = "fromAccountId")]
        admin_id: u64,
        #[serde(rename = "fromAccountName")]
        admin_name: String,
        #[serde(rename = "fromAccountSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "29")]
    PublicationRestored {
        #[serde(rename = "unitId")]
        id: u64,
        #[serde(rename = "unitType")]
        kind: RawPublicationKind,
        #[serde(rename = "parentUnitId")]
        parent_id: u64,
        #[serde(rename = "parentPublicationType")]
        parent_kind: RawPublicationKind,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "30")]
    BlockRejected {
        moderation_id: u64,
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "31")]
    ProfileStatusCleared {
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "32")]
    ProfileDescriptionCleared {
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "33")]
    ProfileNameCleared {
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "34")]
    ProfileLinkRemoved {
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "35")]
    PostFandomChanged {
        #[serde(rename = "unitId")]
        post_id: u64,
        old_fandom_id: u64,
        #[serde(rename = "oldLanguageId")]
        old_fandom_language: RawLanguage,
        old_fandom_name: RawLanguage,
        new_fandom_id: u64,
        #[serde(rename = "newLanguageId")]
        new_fandom_language: RawLanguage,
        new_fandom_name: RawLanguage,
        admin_id: u64,
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "36")]
    PublicationBlockedAfterReport {
        #[serde(rename = "blockUnitType")]
        publication_kind: RawPublicationKind,
        moderation_id: u64,
        #[serde(rename = "blockLastUnits")]
        with_last_publications: bool,
        #[serde(rename = "blockAccountDate")]
        banned_until: i64,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "37")]
    AccountMentioned {
        #[serde(rename = "fromAccountId")]
        account_id: u64,
        #[serde(rename = "fromAccountName")]
        account_name: String,
        #[serde(rename = "fromAccountSex")]
        account_gender: RawGender,
        #[serde(rename = "unitId")]
        publication_id: u64,
        #[serde(rename = "unitType")]
        publication_kind: RawPublicationKind,
        #[serde(rename = "tag1")]
        chat_kind: RawChatKind,
        #[serde(rename = "tag2")]
        chat_first_id: u64,
        #[serde(rename = "tag3")]
        chat_second_id: u64,
        text: String,
    },
    #[serde(rename = "39")]
    PostMultilingualDisabled {
        moderation_id: u64,
        moderator_name: String,
        #[serde(rename = "moderatorSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "41")]
    PostClosed {
        moderation_id: u64,
        moderator_name: String,
        #[serde(rename = "moderatorSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "42")]
    PostOpened {
        moderation_id: u64,
        moderator_name: String,
        #[serde(rename = "moderatorSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "43")]
    RubricNameChanged {
        #[serde(rename = "rubricId")]
        id: u64,
        #[serde(rename = "rubricOldName")]
        old_name: String,
        #[serde(rename = "rubricNewName")]
        new_name: String,
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        moderation_id: u64,
        #[serde(rename = "adminId")]
        moderator_id: u64,
        #[serde(rename = "adminName")]
        moderator_name: String,
        #[serde(rename = "adminSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "44")]
    RubricOwnerTransferred {
        #[serde(rename = "rubricId")]
        id: u64,
        #[serde(rename = "rubricName")]
        name: String,
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        new_owner_id: u64,
        new_owner_name: String,
        moderation_id: u64,
        #[serde(rename = "adminId")]
        moderator_id: u64,
        #[serde(rename = "adminName")]
        moderator_name: String,
        #[serde(rename = "adminSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "45")]
    RubricOwnerAssigned {
        #[serde(rename = "rubricId")]
        id: u64,
        #[serde(rename = "rubricName")]
        name: String,
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        moderation_id: u64,
        #[serde(rename = "adminId")]
        moderator_id: u64,
        #[serde(rename = "adminName")]
        moderator_name: String,
        #[serde(rename = "adminSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "46")]
    RubricRemoved {
        #[serde(rename = "rubricId")]
        id: u64,
        #[serde(rename = "rubricName")]
        name: String,
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        moderation_id: u64,
        #[serde(rename = "adminId")]
        moderator_id: u64,
        #[serde(rename = "adminName")]
        moderator_name: String,
        #[serde(rename = "adminSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "47")]
    RubricKarmaCoefChanged {
        #[serde(rename = "rubricId")]
        id: u64,
        #[serde(rename = "rubricName")]
        name: String,
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        #[serde(rename = "newCof")]
        new_coef: f64,
        #[serde(rename = "cofChange")]
        coef_change: f64,
    },
    #[serde(rename = "48")]
    PublicationReacted {
        #[serde(rename = "reactionIndex")]
        index: i64,
        account_id: u64,
        account_name: String,
        #[serde(rename = "accountSex")]
        account_gender: RawGender,
        #[serde(rename = "unitId")]
        publication_id: u64,
        #[serde(rename = "unitType")]
        publication_kind: RawPublicationKind,
        #[serde(rename = "parentUnitId")]
        parent_id: u64,
        #[serde(rename = "parentUnitType")]
        parent_kind: RawPublicationKind,
    },
    #[serde(rename = "49")]
    PostRelayTurnAssigned {
        #[serde(rename = "fromAccountId")]
        account_id: u64,
        #[serde(rename = "fromAccountName")]
        account_name: String,
        #[serde(rename = "fromAccountSex")]
        account_gender: RawGender,
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        fandom_id: u64,
        #[serde(rename = "fandomLanguageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
    },
    #[serde(rename = "50")]
    PostRelayTurnMissed {
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        fandom_id: u64,
        #[serde(rename = "fandomLanguageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        #[serde(rename = "newAccountId")]
        next_account_id: u64,
        #[serde(rename = "newAccountName")]
        next_account_name: String,
        #[serde(rename = "newAccountSex")]
        next_account_gender: RawGender,
    },
    #[serde(rename = "51")]
    PostRelayPostCreated {
        post_id: u64,
        fandom_id: u64,
        #[serde(rename = "fandomLanguageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        #[serde(rename = "activityId")]
        relay_id: u64,
        #[serde(rename = "activityName")]
        relay_name: String,
    },
    #[serde(rename = "52")]
    PostRelayTurnRejected {
        #[serde(rename = "rejectedAccountId")]
        account_id: u64,
        #[serde(rename = "rejectedAccountName")]
        account_name: String,
        #[serde(rename = "rejectedAccountSex")]
        account_gender: RawGender,
        #[serde(rename = "activityId")]
        id: u64,
        #[serde(rename = "activityName")]
        name: String,
        fandom_id: u64,
        #[serde(rename = "fandomLanguageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        #[serde(rename = "newAccountId")]
        next_account_id: u64,
        #[serde(rename = "newAccountName")]
        next_account_name: String,
        #[serde(rename = "newAccountSex")]
        next_account_gender: RawGender,
    },
    #[serde(rename = "53")]
    FandomCuratorAssigned {
        #[serde(rename = "oldAccountId")]
        old_curator_id: u64,
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        #[serde(rename = "adminAcccountId")]
        admin_id: u64,
        #[serde(rename = "adminAcccountName")]
        admin_name: String,
        #[serde(rename = "adminAcccountSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "54")]
    FandomCuratorRevoked {
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        #[serde(rename = "adminAcccountId")]
        admin_id: u64,
        #[serde(rename = "adminAcccountName")]
        admin_name: String,
        #[serde(rename = "adminAcccountSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "56")]
    DonationProcessed {
        #[serde(rename = "sum")]
        amount: u64,
    },
    #[serde(rename = "57")]
    EffectApplied {
        #[serde(rename = "mAccEffect")]
        effect: RawEffect,
    },
    #[serde(rename = "58")]
    EffectRemoved {
        #[serde(rename = "effectId")]
        id: u64,
        #[serde(rename = "effectIndex")]
        kind: RawEffectKind,
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "61")]
    PostImagesPurged {
        post_id: u64,
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "62")]
    FandomRemovalRejected {
        fandom_id: u64,
        #[serde(rename = "languageId")]
        fandom_language: RawLanguage,
        fandom_name: String,
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "63")]
    AdminActionRejected {
        // #[serde(rename = "mAdminVote")]
        // action: RawAdminAction,
        #[serde(rename = "cancelAdminAccount")]
        rejected_by: RawAccount,
        #[serde(rename = "actionAdminAccount")]
        created_by: RawAccount,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "64")]
    AccountTargetAdminActionRejected {
        // #[serde(rename = "mAdminVote")]
        // action: RawAdminAction,
        #[serde(rename = "cancelAdminAccount")]
        rejected_by: RawAccount,
        #[serde(rename = "actionAdminAccount")]
        created_by: RawAccount,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "65")]
    RubricFandomChanged {
        #[serde(rename = "rubricId")]
        id: u64,
        #[serde(rename = "rubricName")]
        name: String,
        moderation_id: u64,
        admin_id: u64,
        admin_name: String,
        #[serde(rename = "adminSex")]
        admin_gender: RawGender,
        #[serde(rename = "srcFandomId")]
        old_fandom_id: u64,
        #[serde(rename = "srcLanguageId")]
        old_fandom_language: RawLanguage,
        #[serde(rename = "srcFandomName")]
        old_fandom_name: String,
        #[serde(rename = "destFandomId")]
        new_fandom_id: u64,
        #[serde(rename = "destLanguageId")]
        new_fandom_language: RawLanguage,
        #[serde(rename = "destFandomName")]
        new_fandom_name: String,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(rename = "66")]
    AccountUnfollowed {
        #[serde(rename = "J_ACCOUNT_ID")]
        account_id: u64,
        #[serde(rename = "J_ACCOUNT_NAME")]
        account_name: String,
        #[serde(rename = "accountSex")]
        account_gender: RawGender,
    },
    #[serde(rename = "67")]
    PostNsfwToggled {
        #[serde(rename = "nsfw")]
        is_nsfw: bool,
        moderation_id: u64,
        moderator_name: String,
        #[serde(rename = "moderatorSex")]
        moderator_gender: RawGender,
        #[serde(rename = "comment")]
        reason: String,
    },
    #[serde(other)]
    Unknown,
}

impl TryFrom<AnyRawNotification> for AnyNotification {
    type Error = Error;

    fn try_from(value: AnyRawNotification) -> Result<Self> {
        todo!()
    }
}
