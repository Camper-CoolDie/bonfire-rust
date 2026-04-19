mod feed;
mod starter_quest;
mod style;

use chrono::{Timelike, Utc};
pub(crate) use feed::RawKind as RawFeedKind;
use serde::{Deserialize, Serialize};
pub(crate) use starter_quest::RawStage as RawStarterQuestStage;
pub(crate) use style::{RawInterfaceKind, RawTheme};

use crate::models::Settings;
use crate::models::notification::Filter;
use crate::models::publication::{AccountFilter, FandomFilter};
use crate::models::settings::{Feed, Notifications, StarterQuest, StarterQuestStage, Style};
use crate::requests::raw::conversions::{naive_time_from_parts, timestamp_from_millis};
use crate::requests::raw::publication::RawFavoritesFolder;
use crate::requests::raw::{RawCategory, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawSettings {
    pub theme: RawTheme,
    #[serde(rename = "interfaceType")]
    pub interface_kind: RawInterfaceKind,
    #[serde(rename = "postFontSize")]
    pub font_size_sp: u32,
    #[serde(rename = "styleSquare")]
    pub square_avatars: bool,
    #[serde(rename = "styleCorned")]
    pub avatars_corner_radius_dp: u32,
    #[serde(rename = "isProfileListStyle")]
    pub show_new_account_interface: bool,
    #[serde(rename = "useNicknameColors")]
    pub show_custom_account_name_colors: bool,
    #[serde(rename = "styleHolidayEffects")]
    pub show_screen_effects_on_holidays: bool,
    #[serde(rename = "fandomBackground")]
    pub show_background_in_chats: bool,
    #[serde(rename = "styleChatCorned")]
    pub chat_messages_corner_radius_dp: u32,
    #[serde(rename = "styleNewYearAvatars")]
    pub show_custom_account_avatars_on_new_year: bool,
    #[serde(rename = "styleNewYearProfileAnimation")]
    pub show_account_animation_on_new_year: bool,
    #[serde(rename = "styleNewYearSnow")]
    pub new_year_snow_intensity: u32,
    #[serde(rename = "karmaHotness")]
    pub show_karma_hotness: bool,
    #[serde(rename = "postFandomFirst")]
    pub swap_fandom_and_author_in_posts: bool,

    #[serde(rename = "notifications")]
    pub are_notifications_enabled: bool,
    #[serde(rename = "salientTime")]
    pub notifications_disabled_until: i64,
    #[serde(rename = "notificationsSalientAll")]
    pub are_notifications_silent: bool,
    #[serde(rename = "notificationsSalientOnTimeEnabled")]
    pub are_notifications_silenced_daily: bool,
    #[serde(rename = "notificationsSalientOnTimeStartH")]
    pub notifications_silent_starting_hour: u32,
    #[serde(rename = "notificationsSalientOnTimeStartM")]
    pub notifications_silent_starting_minute: u32,
    #[serde(rename = "notificationsSalientOnTimeEndH")]
    pub notifications_silent_ending_hour: u32,
    #[serde(rename = "notificationsSalientOnTimeEndM")]
    pub notifications_silent_ending_minute: u32,
    #[serde(rename = "autoReadNotifications")]
    pub read_notifications_on_list_opening: bool,
    #[serde(rename = "watchPost")]
    pub watch_post_on_commenting: bool,
    #[serde(rename = "notificationsComments")]
    pub are_comment_notifications_enabled: bool,
    #[serde(rename = "notificationsCommentsAnswers")]
    pub are_comment_answer_notifications_enabled: bool,
    #[serde(rename = "notificationsKarma")]
    pub are_rate_notifications_enabled: bool,
    #[serde(rename = "notificationsFollows")]
    pub are_follow_notifications_enabled: bool,
    #[serde(rename = "notificationsImportant")]
    pub are_important_post_notifications_enabled: bool,
    #[serde(rename = "notificationsFollowsPosts")]
    pub are_followed_post_notifications_enabled: bool,
    #[serde(rename = "notificationsAchievements")]
    pub are_achievement_notifications_enabled: bool,
    #[serde(rename = "notificationsChatMessages")]
    pub are_chat_message_notifications_enabled: bool,
    #[serde(rename = "notificationsChatAnswers")]
    pub are_chat_message_answer_notifications_enabled: bool,
    #[serde(rename = "notificationsPM")]
    pub are_direct_chat_message_notifications_enabled: bool,
    #[serde(rename = "notificationsOther")]
    pub are_other_notifications_enabled: bool,

    pub feed_languages: Vec<RawLanguage>,
    pub feed_categories: Vec<RawCategory>,
    #[serde(rename = "feedImportant")]
    pub show_important_in_feed: bool,
    #[serde(rename = "feedClosed")]
    pub show_closed_in_feed: bool,
    #[serde(rename = "feedOrder_v2")]
    pub feed_kinds: Vec<RawFeedKind>,

    #[serde(rename = "appLanguage")]
    pub language: String,

    #[serde(rename = "profileFilterPosts")]
    pub show_posts_in_accounts: bool,
    #[serde(rename = "profileFilterComments")]
    pub show_comments_in_accounts: bool,
    #[serde(rename = "profileFilterChatMessages")]
    pub show_chat_messages_in_accounts: bool,
    #[serde(rename = "profileFilterEvents")]
    pub show_events_in_accounts: bool,
    #[serde(rename = "profileFilterStickers")]
    pub show_sticker_events_in_accounts: bool,
    #[serde(rename = "profileFilterModerations")]
    pub show_moderations_in_accounts: bool,

    #[serde(rename = "fandomFilterModerationsPosts")]
    pub show_posts_in_fandoms: bool,
    #[serde(rename = "fandomFilterOnlyImportant")]
    pub show_only_important_in_fandoms: bool,
    #[serde(rename = "fandomFilterAdministrations")]
    pub show_events_in_fandoms: bool,
    #[serde(rename = "fandomFilterModerations")]
    pub show_moderations_in_fandoms: bool,
    #[serde(rename = "fandomFilterModerationsBlocks")]
    pub show_blocks_in_fandoms: bool,

    #[serde(rename = "notificationsFilterComments")]
    pub show_comment_notifications: bool,
    #[serde(rename = "notificationsFilterAnswers")]
    pub show_answer_notifications: bool,
    #[serde(rename = "notificationsFilterKarma")]
    pub show_rate_notifications: bool,
    #[serde(rename = "notificationsFilterFollows")]
    pub show_follow_notifications: bool,
    #[serde(rename = "notificationsFilterImportant")]
    pub show_important_post_notifications: bool,
    #[serde(rename = "notificationsFilterFollowsPublications")]
    pub show_followed_post_notifications: bool,
    #[serde(rename = "notificationsFilterAchievements")]
    pub show_achievement_notifications: bool,
    #[serde(rename = "notificationsFilterOther")]
    pub show_other_notifications: bool,

    #[serde(rename = "bookmarksFolders")]
    pub favorites_folders: Vec<RawFavoritesFolder>,
    #[serde(rename = "storyQuestIndex")]
    pub starter_quest_stage: RawStarterQuestStage,
    #[serde(rename = "storyQuestProgress")]
    pub starter_quest_progress: usize,
    #[serde(rename = "helloIsShowed")]
    pub is_registration_completed: bool,
    #[serde(rename = "rulesIsShowed")]
    pub are_terms_accepted: bool,
    #[serde(rename = "lvlDialogLvl")]
    pub last_congratulated_level: i64,
    #[serde(rename = "anonRates")]
    pub rate_anonymously: bool,
    #[serde(rename = "voiceMessagesAutoSend")]
    pub send_voice_after_recording: bool,
    #[serde(rename = "voiceMessagesAutoLock")]
    pub lock_voice_recording: bool,
    #[serde(rename = "voiceMessagesIgnore")]
    pub block_voice_messages_in_direct_chats: bool,
    #[serde(rename = "allowAddingToConferences")]
    pub allow_adding_to_groups: bool,
    #[serde(rename = "viewedChats")]
    pub intro_shown_for_sub_chat_ids: Vec<u64>,
    #[serde(rename = "hideBlacklistedPubs")]
    pub hide_hidden_publications: bool,
    #[serde(rename = "fandomNSFW")]
    pub closed_alert_shown_for_fandom_ids: Vec<u64>,
    pub show_nsfw_posts: bool,
    #[serde(rename = "userActivitiesAutoSubscribe")]
    pub subscribe_to_post_relays_when_posting: bool,
    #[serde(rename = "userActivitiesAllowed_all")]
    pub allow_post_passes_from_everyone: bool,
    #[serde(rename = "userActivitiesAllowed_followedFandoms")]
    pub allow_post_passes_from_subscribed_fandoms: bool,
    #[serde(rename = "userActivitiesAllowed_followedUsers")]
    pub allow_post_passes_from_followed_accounts: bool,
    #[serde(rename = "adminReportsLanguages")]
    pub reports_languages: Vec<RawLanguage>,
    #[serde(rename = "longPlusFandomId")]
    pub quick_post_fandom_id: u64,
    #[serde(rename = "longPlusFandomLanguageId")]
    pub quick_post_fandom_language: RawLanguage,
    #[serde(rename = "longPlusFandomName")]
    pub quick_post_fandom_name: String,
    #[serde(rename = "fastPublicationFandomId")]
    pub quick_image_fandom_id: u64,
    #[serde(rename = "fastPublicationFandomLanguageId")]
    pub quick_image_fandom_language: RawLanguage,
    #[serde(rename = "fastPublicationFandomName")]
    pub quick_image_fandom_name: String,
}

impl TryFrom<RawSettings> for Settings {
    type Error = Error;

    #[expect(clippy::too_many_lines)]
    fn try_from(value: RawSettings) -> Result<Self> {
        Ok(Self {
            style: Style {
                theme: value.theme.into(),
                interface_kind: value.interface_kind.try_into()?,
                font_size_sp: value.font_size_sp,
                round_avatars: !value.square_avatars,
                avatars_corner_radius_dp: value.avatars_corner_radius_dp,
                show_new_account_interface: value.show_new_account_interface,
                show_custom_account_name_colors: value.show_custom_account_name_colors,
                show_screen_effects_on_holidays: value.show_screen_effects_on_holidays,
                show_background_in_chats: value.show_background_in_chats,
                chat_messages_corner_radius_dp: value.chat_messages_corner_radius_dp,
                show_custom_account_avatars_on_new_year: value
                    .show_custom_account_avatars_on_new_year,
                show_account_animation_on_new_year: value.show_account_animation_on_new_year,
                new_year_snow_intensity: value.new_year_snow_intensity,
                show_karma_hotness: value.show_karma_hotness,
                swap_fandom_and_author_in_posts: value.swap_fandom_and_author_in_posts,
            },
            notifications: Notifications {
                are_enabled: value.are_notifications_enabled,
                disabled_until: match value.notifications_disabled_until {
                    0 => None,
                    timestamp => Some(timestamp_from_millis(timestamp)?),
                }
                .filter(|date| *date > Utc::now()),
                are_silent: value.are_notifications_silent,
                are_silenced_daily: value.are_notifications_silenced_daily,
                silent_during: (
                    naive_time_from_parts(
                        value.notifications_silent_starting_hour,
                        value.notifications_silent_starting_minute,
                    )?,
                    naive_time_from_parts(
                        value.notifications_silent_ending_hour,
                        value.notifications_silent_ending_minute,
                    )?,
                ),
                read_on_list_opening: value.read_notifications_on_list_opening,
                watch_post_on_commenting: value.watch_post_on_commenting,
                are_comments_enabled: value.are_comment_notifications_enabled,
                are_comment_answers_enabled: value.are_comment_answer_notifications_enabled,
                are_rates_enabled: value.are_rate_notifications_enabled,
                are_follows_enabled: value.are_follow_notifications_enabled,
                are_important_posts_enabled: value.are_important_post_notifications_enabled,
                are_followed_posts_enabled: value.are_followed_post_notifications_enabled,
                are_achievements_enabled: value.are_achievement_notifications_enabled,
                are_chat_messages_enabled: value.are_chat_message_notifications_enabled,
                are_chat_message_answers_enabled: value
                    .are_chat_message_answer_notifications_enabled,
                are_direct_chat_messages_enabled: value
                    .are_direct_chat_message_notifications_enabled,
                are_other_enabled: value.are_other_notifications_enabled,
            },
            feed: Feed {
                languages: value
                    .feed_languages
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_>>()?,
                categories: value.feed_categories.into_iter().map(Into::into).collect(),
                show_important: value.show_important_in_feed,
                show_closed: value.show_closed_in_feed,
                kinds: value
                    .feed_kinds
                    .into_iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_>>()?,
            },
            language: RawLanguage::from(value.language).try_into()?,
            account_publications_filter: AccountFilter {
                posts: value.show_posts_in_accounts,
                comments: value.show_comments_in_accounts,
                chat_messages: value.show_chat_messages_in_accounts,
                events: value.show_events_in_accounts,
                sticker_events: value.show_sticker_events_in_accounts,
                moderations: value.show_moderations_in_accounts,
            },
            fandom_publications_filter: FandomFilter {
                posts: value.show_posts_in_fandoms,
                only_important: value.show_only_important_in_fandoms,
                events: value.show_events_in_fandoms,
                moderations: value.show_moderations_in_fandoms,
                blocks: value.show_blocks_in_fandoms,
            },
            notifications_filter: Filter {
                comments: value.show_comment_notifications,
                answers: value.show_answer_notifications,
                rates: value.show_rate_notifications,
                follows: value.show_follow_notifications,
                important_posts: value.show_important_post_notifications,
                followed_posts: value.show_followed_post_notifications,
                achievements: value.show_achievement_notifications,
                other: value.show_other_notifications,
            },
            favorites_folders: value
                .favorites_folders
                .into_iter()
                .map(|folder| (folder.id, folder.name))
                .collect(),
            starter_quest: Option::<StarterQuestStage>::from(value.starter_quest_stage).map(
                |stage| StarterQuest {
                    stage,
                    progress: value.starter_quest_progress,
                },
            ),
            is_registration_completed: value.is_registration_completed,
            are_terms_accepted: value.are_terms_accepted,
            last_congratulated_level: value.last_congratulated_level,
            rate_anonymously: value.rate_anonymously,
            send_voice_after_recording: value.send_voice_after_recording,
            lock_voice_recording: value.lock_voice_recording,
            block_voice_messages_in_direct_chats: value.block_voice_messages_in_direct_chats,
            allow_adding_to_groups: value.allow_adding_to_groups,
            intro_shown_for_sub_chat_ids: value.intro_shown_for_sub_chat_ids,
            show_hidden_publications: !value.hide_hidden_publications,
            closed_alert_shown_for_fandom_ids: value.closed_alert_shown_for_fandom_ids,
            show_nsfw_posts: value.show_nsfw_posts,
            subscribe_to_post_relays_when_posting: value.subscribe_to_post_relays_when_posting,
            allow_post_passes_from_everyone: value.allow_post_passes_from_everyone,
            allow_post_passes_from_subscribed_fandoms: value
                .allow_post_passes_from_subscribed_fandoms,
            allow_post_passes_from_followed_accounts: value
                .allow_post_passes_from_followed_accounts,
            reports_languages: value
                .reports_languages
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
            quick_post_fandom_id: value.quick_post_fandom_id,
            quick_post_fandom_language: value.quick_post_fandom_language.try_into()?,
            quick_post_fandom_name: value.quick_post_fandom_name,
            quick_image_fandom_id: value.quick_image_fandom_id,
            quick_image_fandom_language: value.quick_image_fandom_language.try_into()?,
            quick_image_fandom_name: value.quick_image_fandom_name,
        })
    }
}

impl TryFrom<Settings> for RawSettings {
    type Error = Error;

    #[expect(clippy::too_many_lines)]
    fn try_from(value: Settings) -> Result<Self> {
        let (starter_quest_stage, starter_quest_progress) = value
            .starter_quest
            .map_or((RawStarterQuestStage::Completed, 0), |quest| {
                (RawStarterQuestStage::from(quest.stage), quest.progress)
            });

        Ok(Self {
            theme: value.style.theme.into(),
            interface_kind: value.style.interface_kind.into(),
            font_size_sp: value.style.font_size_sp,
            square_avatars: !value.style.round_avatars,
            avatars_corner_radius_dp: value.style.avatars_corner_radius_dp,
            show_new_account_interface: value.style.show_new_account_interface,
            show_custom_account_name_colors: value.style.show_custom_account_name_colors,
            show_screen_effects_on_holidays: value.style.show_screen_effects_on_holidays,
            show_background_in_chats: value.style.show_background_in_chats,
            chat_messages_corner_radius_dp: value.style.chat_messages_corner_radius_dp,
            show_custom_account_avatars_on_new_year: value
                .style
                .show_custom_account_avatars_on_new_year,
            show_account_animation_on_new_year: value.style.show_account_animation_on_new_year,
            new_year_snow_intensity: value.style.new_year_snow_intensity,
            show_karma_hotness: value.style.show_karma_hotness,
            swap_fandom_and_author_in_posts: value.style.swap_fandom_and_author_in_posts,

            are_notifications_enabled: value.notifications.are_enabled,
            notifications_disabled_until: match value.notifications.disabled_until {
                None => 0,
                Some(timestamp) => timestamp.timestamp_millis(),
            },
            are_notifications_silent: value.notifications.are_silent,
            are_notifications_silenced_daily: value.notifications.are_silenced_daily,
            notifications_silent_starting_hour: value.notifications.silent_during.0.hour(),
            notifications_silent_starting_minute: value.notifications.silent_during.0.minute(),
            notifications_silent_ending_hour: value.notifications.silent_during.1.hour(),
            notifications_silent_ending_minute: value.notifications.silent_during.1.minute(),
            read_notifications_on_list_opening: value.notifications.read_on_list_opening,
            watch_post_on_commenting: value.notifications.watch_post_on_commenting,
            are_comment_notifications_enabled: value.notifications.are_comments_enabled,
            are_comment_answer_notifications_enabled: value
                .notifications
                .are_comment_answers_enabled,
            are_rate_notifications_enabled: value.notifications.are_rates_enabled,
            are_follow_notifications_enabled: value.notifications.are_follows_enabled,
            are_important_post_notifications_enabled: value
                .notifications
                .are_important_posts_enabled,
            are_followed_post_notifications_enabled: value.notifications.are_followed_posts_enabled,
            are_achievement_notifications_enabled: value.notifications.are_achievements_enabled,
            are_chat_message_notifications_enabled: value.notifications.are_chat_messages_enabled,
            are_chat_message_answer_notifications_enabled: value
                .notifications
                .are_chat_message_answers_enabled,
            are_direct_chat_message_notifications_enabled: value
                .notifications
                .are_direct_chat_messages_enabled,
            are_other_notifications_enabled: value.notifications.are_other_enabled,

            feed_languages: value.feed.languages.into_iter().map(Into::into).collect(),
            feed_categories: value.feed.categories.into_iter().map(Into::into).collect(),
            show_important_in_feed: value.feed.show_important,
            show_closed_in_feed: value.feed.show_closed,
            feed_kinds: value.feed.kinds.into_iter().map(Into::into).collect(),

            language: value
                .language
                .map_or(RawLanguage::UnknownString(String::new()), |language| {
                    RawLanguage::from(language)
                })
                .try_into()?,

            show_posts_in_accounts: value.account_publications_filter.posts,
            show_comments_in_accounts: value.account_publications_filter.comments,
            show_chat_messages_in_accounts: value.account_publications_filter.chat_messages,
            show_events_in_accounts: value.account_publications_filter.events,
            show_sticker_events_in_accounts: value.account_publications_filter.sticker_events,
            show_moderations_in_accounts: value.account_publications_filter.moderations,

            show_posts_in_fandoms: value.fandom_publications_filter.posts,
            show_only_important_in_fandoms: value.fandom_publications_filter.only_important,
            show_events_in_fandoms: value.fandom_publications_filter.events,
            show_moderations_in_fandoms: value.fandom_publications_filter.moderations,
            show_blocks_in_fandoms: value.fandom_publications_filter.blocks,

            show_comment_notifications: value.notifications_filter.comments,
            show_answer_notifications: value.notifications_filter.answers,
            show_rate_notifications: value.notifications_filter.rates,
            show_follow_notifications: value.notifications_filter.follows,
            show_important_post_notifications: value.notifications_filter.important_posts,
            show_followed_post_notifications: value.notifications_filter.followed_posts,
            show_achievement_notifications: value.notifications_filter.achievements,
            show_other_notifications: value.notifications_filter.other,

            favorites_folders: value
                .favorites_folders
                .into_iter()
                .map(|(id, name)| RawFavoritesFolder { id, name })
                .collect(),
            starter_quest_stage,
            starter_quest_progress,
            is_registration_completed: value.is_registration_completed,
            are_terms_accepted: value.are_terms_accepted,
            last_congratulated_level: value.last_congratulated_level,
            rate_anonymously: value.rate_anonymously,
            send_voice_after_recording: value.send_voice_after_recording,
            lock_voice_recording: value.lock_voice_recording,
            block_voice_messages_in_direct_chats: value.block_voice_messages_in_direct_chats,
            allow_adding_to_groups: value.allow_adding_to_groups,
            intro_shown_for_sub_chat_ids: value.intro_shown_for_sub_chat_ids,
            hide_hidden_publications: !value.show_hidden_publications,
            closed_alert_shown_for_fandom_ids: value.closed_alert_shown_for_fandom_ids,
            show_nsfw_posts: value.show_nsfw_posts,
            subscribe_to_post_relays_when_posting: value.subscribe_to_post_relays_when_posting,
            allow_post_passes_from_everyone: value.allow_post_passes_from_everyone,
            allow_post_passes_from_subscribed_fandoms: value
                .allow_post_passes_from_subscribed_fandoms,
            allow_post_passes_from_followed_accounts: value
                .allow_post_passes_from_followed_accounts,
            reports_languages: value
                .reports_languages
                .into_iter()
                .map(Into::into)
                .collect(),
            quick_post_fandom_id: value.quick_post_fandom_id,
            quick_post_fandom_language: value
                .quick_post_fandom_language
                .map_or(RawLanguage::UnknownInteger(0), Into::into),
            quick_post_fandom_name: value.quick_post_fandom_name,
            quick_image_fandom_id: value.quick_image_fandom_id,
            quick_image_fandom_language: value
                .quick_image_fandom_language
                .map_or(RawLanguage::UnknownInteger(0), Into::into),
            quick_image_fandom_name: value.quick_image_fandom_name,
        })
    }
}
