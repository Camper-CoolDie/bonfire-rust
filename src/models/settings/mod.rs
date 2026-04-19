mod feed;
mod notifications;
mod starter_quest;
mod style;

use std::collections::HashMap;

pub use feed::{Feed, Kind as FeedKind};
pub use notifications::Notifications;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub use starter_quest::{Stage as StarterQuestStage, StarterQuest};
pub use style::{InterfaceKind, Style, Theme, ThemeColor};

use crate::client::Request as _;
use crate::models::notification::Filter as NotificationsFilter;
use crate::models::publication::{AccountFilter, FandomFilter};
use crate::models::{Config, Language};
use crate::requests::other::SaveSettingsRequest;
use crate::{Client, Result};

/// Represents the user's personal settings for the application.
///
/// This struct consolidates various configuration options, from visual themes to notification
/// preferences and feature-specific toggles.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Settings {
    /// Visual style settings
    pub style: Style,
    /// Notification preferences
    pub notifications: Notifications,
    /// Feed settings
    pub feed: Feed,
    /// The preferred language for the application, or `None` if the language is taken from the
    /// system
    pub language: Option<Language>,
    /// Filter for account publications
    pub account_publications_filter: AccountFilter,
    /// Filter for fandom publications
    pub fandom_publications_filter: FandomFilter,
    /// Filter for notifications
    pub notifications_filter: NotificationsFilter,
    /// Favorites folders indexed by folder ID and name.
    ///
    /// The key is the folder ID (`u64`) used to retrieve posts from that folder. The value is the
    /// folder name (`String`).
    pub favorites_folders: HashMap<u64, String>,
    /// The user's starter quest progress, if any
    pub starter_quest: Option<StarterQuest>,
    /// Indicates whether the user has completed the registration process.
    ///
    /// This includes selecting a name, gender, avatar, favorite categories, and agreeing to
    /// Bonfire rules, privacy policies, and terms of service.
    pub is_registration_completed: bool,
    /// Whether the user has accepted Bonfire rules, terms of service, and privacy policy
    pub are_terms_accepted: bool,
    /// The latest level for which a congratulation dialog was shown
    pub last_congratulated_level: i64,
    /// Whether to automatically place anonymous rates on publications
    pub rate_anonymously: bool,
    /// Whether to send voice messages immediately after recording is completed
    pub send_voice_after_recording: bool,
    /// Whether to continue recording voice messages when the record button is released
    pub lock_voice_recording: bool,
    /// Whether to block voice messages in direct chats
    pub block_voice_messages_in_direct_chats: bool,
    /// Whether to allow being added to groups by other users
    pub allow_adding_to_groups: bool,
    /// A list of sub-chat IDs for which the user has already viewed the introductory message
    pub intro_shown_for_sub_chat_ids: Vec<u64>,
    /// Whether to show publications that originate from a blocked account
    pub show_hidden_publications: bool,
    /// List of fandom IDs for which the "fandom is closed" alert has been shown
    pub closed_alert_shown_for_fandom_ids: Vec<u64>,
    /// Whether to show NSFW posts in the feed.
    ///
    /// NSFW posts can only be shown if the profile has
    /// [`Profile::is_nsfw_allowed`][crate::models::Profile::is_nsfw_allowed] set to `true`.
    pub show_nsfw_posts: bool,
    /// Whether to automatically subscribe to post relays when posting
    pub subscribe_to_post_relays_when_posting: bool,
    /// Whether to allow post passes from everyone
    pub allow_post_passes_from_everyone: bool,
    /// Whether to allow post passes from accounts in subscribed fandoms
    pub allow_post_passes_from_subscribed_fandoms: bool,
    /// Whether to allow post passes from followed accounts
    pub allow_post_passes_from_followed_accounts: bool,
    /// The list of languages for which publication reports will be shown, or empty if no language
    /// has been selected yet
    pub reports_languages: Vec<Language>,
    /// The ID of the fandom used for quick post creation.
    ///
    /// Quick post creation is an action that skips the fandom selection screen when creating
    /// a new post draft.
    pub quick_post_fandom_id: u64,
    /// The language of the fandom used for quick post creation, or `None` if it matches
    /// [`language`][Self::language]
    pub quick_post_fandom_language: Option<Language>,
    /// The name of the fandom used for quick post creation
    pub quick_post_fandom_name: String,
    /// The ID of the fandom used for quick image publication.
    ///
    /// Quick image publication provides an option to post an image with attached text in one click
    /// when shared to Bonfire.
    pub quick_image_fandom_id: u64,
    /// The language of the fandom used for quick image publication, or `None` if it matches
    /// [`language`][Self::language]
    pub quick_image_fandom_language: Option<Language>,
    /// The name of the fandom used for quick image publication
    pub quick_image_fandom_name: String,
}
impl Settings {
    /// Creates a new `Settings` instance with default values, customized using the provided
    /// configuration.
    #[must_use]
    pub fn new(config: &Config) -> Self {
        Self {
            quick_post_fandom_id: config.quick_post_fandom_id,
            quick_post_fandom_name: config.quick_post_fandom_name.clone(),
            quick_image_fandom_id: config.quick_image_fandom_id,
            quick_image_fandom_name: config.quick_image_fandom_name.clone(),
            ..Default::default()
        }
    }

    /// Sends the current settings to the server to update the user's account settings.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn save(&self, client: &Client) -> Result<()> {
        SaveSettingsRequest::new(self.clone())?
            .send_request(client)
            .await?;
        Ok(())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            style: Style::default(),
            notifications: Notifications::default(),
            feed: Feed::default(),
            language: None,
            account_publications_filter: AccountFilter::default(),
            fandom_publications_filter: FandomFilter::default(),
            notifications_filter: NotificationsFilter::default(),
            favorites_folders: HashMap::new(),
            starter_quest: Some(StarterQuest::default()),
            is_registration_completed: false,
            are_terms_accepted: false,
            last_congratulated_level: 1,
            rate_anonymously: false,
            send_voice_after_recording: false,
            lock_voice_recording: false,
            block_voice_messages_in_direct_chats: false,
            allow_adding_to_groups: true,
            intro_shown_for_sub_chat_ids: Vec::new(),
            show_hidden_publications: true,
            closed_alert_shown_for_fandom_ids: Vec::new(),
            show_nsfw_posts: true,
            subscribe_to_post_relays_when_posting: true,
            allow_post_passes_from_everyone: true,
            allow_post_passes_from_subscribed_fandoms: true,
            allow_post_passes_from_followed_accounts: true,
            reports_languages: Vec::new(),
            quick_post_fandom_id: 0,
            quick_post_fandom_language: None,
            quick_post_fandom_name: String::new(),
            quick_image_fandom_id: 0,
            quick_image_fandom_language: None,
            quick_image_fandom_name: String::new(),
        }
    }
}
