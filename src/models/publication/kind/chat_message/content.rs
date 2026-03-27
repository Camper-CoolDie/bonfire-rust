use std::time::Duration;

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::chat::MemberRole;
use crate::models::{Gender, ImageRef};

/// The maximum allowed size in bytes for a static chat message image.
pub const CHAT_MESSAGE_IMAGE_MAX_SIZE: usize = 256 * 1024;
/// The maximum allowed dimension (width or height) for a static chat message image.
pub const CHAT_MESSAGE_IMAGE_MAX_DIMENSION: usize = 1080;
/// The maximum allowed size in bytes for a GIF chat message.
pub const CHAT_MESSAGE_GIF_MAX_SIZE: usize = 1024 * 1024;
/// The maximum allowed dimension (width or height) for a GIF chat message.
pub const CHAT_MESSAGE_GIF_MAX_DIMENSION: usize = 400;
/// The maximum number of static images that can be included in a single chat message.
pub const CHAT_MESSAGE_IMAGES_MAX_COUNT: usize = 5;
/// The maximum allowed duration for a voice message.
pub const CHAT_MESSAGE_VOICE_MAX_DURATION: Duration = Duration::from_secs(20);

/// Represents the content of a chat message.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub enum Content {
    /// No specific content
    #[default]
    Text,
    /// A single static image
    Image(ImageRef),
    /// An animated GIF image
    Gif {
        /// The first frame of the GIF as a static image
        first_frame: ImageRef,
        /// The animated GIF itself
        animated: ImageRef,
    },
    /// A collection of multiple static images
    Images(Vec<ImageRef>),
    /// A sticker
    Sticker {
        /// The unique identifier of the sticker
        id: u64,
        /// The static image representation of the sticker
        image: ImageRef,
        /// The GIF representation of the sticker, if available
        gif: Option<ImageRef>,
    },
    /// A voice message
    Voice {
        /// The unique identifier of the voice message
        id: u64,
        /// The URI from which this voice message can be downloaded
        uri: String,
        /// The duration of the voice message
        duration: Duration,
        /// The waveform data of the voice message
        waveform: Vec<u64>,
    },
    /// An event indicating a message was blocked by moderators. Can appear only inside a fandom
    /// chat
    BlockEvent {
        /// The ID of the account that performed the block
        by_account_id: u64,
        /// The name of the account that performed the block
        by_account_name: String,
        /// The gender of the account that performed the block
        by_account_gender: Gender,
        /// The name of the user whose message was blocked
        name: String,
        /// The reason for blocking the message
        reason: String,
        /// The ID of the moderation action
        moderation_id: u64,
        /// Indicates if the user was punished
        is_punished: bool,
        /// The date until the user is banned, if applicable
        banned_until: Option<DateTime<Utc>>,
    },
    /// An event indicating a chat was created. Can appear only inside groups
    CreateEvent {
        /// The ID of the account that created the chat
        by_account_id: u64,
        /// The name of the account that created the chat
        by_account_name: String,
        /// The gender of the account that created the chat
        by_account_gender: Gender,
    },
    /// An event indicating a member was added to the chat. Can appear only inside groups
    AddMemberEvent {
        /// The ID of the account that added the member
        by_account_id: u64,
        /// The name of the account that added the member
        by_account_name: String,
        /// The gender of the account that added the member
        by_account_gender: Gender,
        /// The name of the member who was added
        name: String,
    },
    /// An event indicating a member was removed from the chat. Can appear only inside groups
    RemoveMemberEvent {
        /// The ID of the account that removed the member
        by_account_id: u64,
        /// The name of the account that removed the member
        by_account_name: String,
        /// The gender of the account that removed the member
        by_account_gender: Gender,
        /// The name of the member who was removed
        name: String,
    },
    /// An event indicating a member's role was changed. Can appear only inside groups
    ChangeRoleEvent {
        /// The ID of the account that changed the role
        by_account_id: u64,
        /// The name of the account that changed the role
        by_account_name: String,
        /// The gender of the account that changed the role
        by_account_gender: Gender,
        /// The name of the member whose role was changed
        name: String,
        /// The new role assigned to the member
        new_role: MemberRole,
    },
    /// An event indicating a member entered the chat by link. Can appear only inside groups
    EnterEvent {
        /// The ID of the member who entered
        id: u64,
        /// The name of the member who entered
        name: String,
        /// The gender of the member who entered
        gender: Gender,
    },
    /// An event indicating a member left the chat. Can appear only inside groups
    LeaveEvent {
        /// The ID of the member who left
        id: u64,
        /// The name of the member who left
        name: String,
        /// The gender of the member who left
        gender: Gender,
    },
    /// An event indicating the chat was renamed. Can appear only inside groups
    RenameEvent {
        /// The ID of the account that renamed the chat
        by_account_id: u64,
        /// The name of the account that renamed the chat
        by_account_name: String,
        /// The gender of the account that renamed the chat
        by_account_gender: Gender,
        /// The new name of the chat
        new_name: String,
    },
    /// An event indicating the chat icon was changed. Can appear only inside groups
    ChangeIconEvent {
        /// The ID of the account that changed the icon
        by_account_id: u64,
        /// The name of the account that changed the icon
        by_account_name: String,
        /// The gender of the account that changed the icon
        by_account_gender: Gender,
        /// The ID of the new icon image
        image_id: u64,
    },
    /// An event indicating the chat background was changed or removed. Can appear only inside
    /// groups
    ChangeBackgroundEvent {
        /// The ID of the account that changed the background
        by_account_id: u64,
        /// The name of the account that changed the background
        by_account_name: String,
        /// The gender of the account that changed the background
        by_account_gender: Gender,
        /// The ID of the new background image, or `None` if it was removed
        image_id: Option<u64>,
    },
    /// An event indicating chat parameters were changed. Can appear only inside groups
    ChangeParamsEvent {
        /// The ID of the account that changed the parameters
        by_account_id: u64,
        /// The name of the account that changed the parameters
        by_account_name: String,
        /// The gender of the account that changed the parameters
        by_account_gender: Gender,
    },
    /// An unknown content type
    Unknown(i64),
    /// An unknown event type
    UnknownEvent(i64),
}
impl Content {
    /// Returns `true` if this content represents a chat event (e.g.,
    /// [`BlockEvent`][Content::BlockEvent], [`CreateEvent`][Content::CreateEvent]).
    #[must_use]
    pub fn is_event(&self) -> bool {
        matches!(
            self,
            Content::BlockEvent { .. }
                | Content::CreateEvent { .. }
                | Content::AddMemberEvent { .. }
                | Content::RemoveMemberEvent { .. }
                | Content::ChangeRoleEvent { .. }
                | Content::EnterEvent { .. }
                | Content::LeaveEvent { .. }
                | Content::RenameEvent { .. }
                | Content::ChangeIconEvent { .. }
                | Content::ChangeBackgroundEvent { .. }
                | Content::ChangeParamsEvent { .. }
        )
    }
}
