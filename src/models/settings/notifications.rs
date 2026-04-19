use chrono::{DateTime, NaiveTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const SILENT_DURING: (NaiveTime, NaiveTime) = (
    NaiveTime::from_hms_opt(23, 0, 0).unwrap(),
    NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
);

/// Represents the user's notification settings.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Notifications {
    /// Whether notifications are generally enabled
    pub are_enabled: bool,
    /// The timestamp until which notifications are temporarily disabled, if any
    pub disabled_until: Option<DateTime<Utc>>,
    /// Whether notifications are silent
    pub are_silent: bool,
    /// Whether notifications are silenced daily during a specific period
    pub are_silenced_daily: bool,
    /// The time range during which notifications are silenced daily, if
    /// [`are_silenced_daily`][Self::are_silenced_daily] is `true`
    pub silent_during: (NaiveTime, NaiveTime),
    /// Whether notifications are marked as read when opening the notification list
    pub read_on_list_opening: bool,
    /// Whether to watch a post for new comments after commenting on it
    pub watch_post_on_commenting: bool,
    /// Whether comment notifications are enabled
    pub are_comments_enabled: bool,
    /// Whether comment answer (reply) notifications are enabled
    pub are_comment_answers_enabled: bool,
    /// Whether rate notifications are enabled
    pub are_rates_enabled: bool,
    /// Whether follow notifications are enabled
    pub are_follows_enabled: bool,
    /// Whether important post notifications are enabled
    pub are_important_posts_enabled: bool,
    /// Whether notifications for posts from followed accounts are enabled
    pub are_followed_posts_enabled: bool,
    /// Whether achievement notifications are enabled
    pub are_achievements_enabled: bool,
    /// Whether chat message notifications are enabled
    pub are_chat_messages_enabled: bool,
    /// Whether chat message answer (reply) notifications are enabled
    pub are_chat_message_answers_enabled: bool,
    /// Whether direct chat message notifications are enabled
    pub are_direct_chat_messages_enabled: bool,
    /// Whether other types of notifications are enabled
    pub are_other_enabled: bool,
}

impl Default for Notifications {
    fn default() -> Self {
        Self {
            are_enabled: true,
            disabled_until: None,
            are_silent: false,
            are_silenced_daily: true,
            silent_during: SILENT_DURING,
            read_on_list_opening: false,
            watch_post_on_commenting: false,
            are_comments_enabled: true,
            are_comment_answers_enabled: true,
            are_rates_enabled: true,
            are_follows_enabled: true,
            are_important_posts_enabled: true,
            are_followed_posts_enabled: true,
            are_achievements_enabled: true,
            are_chat_messages_enabled: true,
            are_chat_message_answers_enabled: true,
            are_direct_chat_messages_enabled: true,
            are_other_enabled: true,
        }
    }
}
