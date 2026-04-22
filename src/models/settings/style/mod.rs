mod interface_kind;
mod theme;

pub use interface_kind::InterfaceKind;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
pub use theme::{Color as ThemeColor, Theme};

/// Represents the user's visual style settings for the application.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Style {
    /// The selected visual theme
    pub theme: Theme,
    /// The kind of user interface (navigation panel or menu)
    pub interface_kind: InterfaceKind,
    /// Whether avatars should be displayed as round
    pub round_avatars: bool,
    /// The corner radius in density-independent pixels (dp) for avatars if not round
    pub avatars_corner_radius_dp: u32,
    /// Whether to show the new account interface
    pub show_new_account_interface: bool,
    /// Whether to show custom account name colors
    pub show_custom_account_name_colors: bool,
    /// Whether to show screen effects on holidays
    pub show_screen_effects_on_holidays: bool,
    /// Whether to show backgrounds in chats
    pub show_background_in_chats: bool,
    /// The corner radius in density-independent pixels (dp) for chat messages
    pub chat_messages_corner_radius_dp: u32,
    /// Whether to show custom account avatars during New Year celebrations
    pub show_custom_account_avatars_on_new_year: bool,
    /// Whether to show account animations during New Year celebrations
    pub show_account_animation_on_new_year: bool,
    /// The intensity of the New Year snow effect
    pub new_year_snow_intensity: u32,
    /// Whether to show karma hotness indicators
    pub show_karma_hotness: bool,
    /// Whether to swap the fandom and author information in post displays
    pub swap_fandom_and_author_in_posts: bool,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            interface_kind: InterfaceKind::default(),
            round_avatars: false,
            avatars_corner_radius_dp: 10,
            show_new_account_interface: true,
            show_custom_account_name_colors: true,
            show_screen_effects_on_holidays: true,
            show_background_in_chats: true,
            chat_messages_corner_radius_dp: 8,
            show_custom_account_avatars_on_new_year: true,
            show_account_animation_on_new_year: true,
            new_year_snow_intensity: 100,
            show_karma_hotness: true,
            swap_fandom_and_author_in_posts: false,
        }
    }
}
