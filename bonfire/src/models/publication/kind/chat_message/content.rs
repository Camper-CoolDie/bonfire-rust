use std::time::Duration;

use chrono::{DateTime, Utc};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::chat::MemberRole;
use crate::models::{AccountRef, ImageRef, VoiceRef};

/// Represents the content of a chat message.
#[derive(Default, Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
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
    Voice(VoiceRef),
    /// An event indicating a message was blocked by moderators. Can appear only inside a fandom
    /// chat
    BlockEvent {
        target_name: String,
        /// The ID of the moderation action
        moderation_id: u64,
        moderator: AccountRef,
        /// Indicates if the user was punished
        is_punished: bool,
        /// The date until the user is banned, if applicable
        banned_until: Option<DateTime<Utc>>,
        /// The reason for blocking the message
        reason: String,
    },
    /// An event indicating a chat was created. Can appear only inside groups
    CreateEvent { moderator: AccountRef },
    /// An event indicating a member was added to the chat. Can appear only inside groups
    AddMemberEvent {
        target_name: String,
        member: AccountRef,
    },
    /// An event indicating a member was removed from the chat. Can appear only inside groups
    RemoveMemberEvent {
        target_name: String,
        member: AccountRef,
    },
    /// An event indicating a member's role was changed. Can appear only inside groups
    ChangeRoleEvent {
        target_name: String,
        /// The new role assigned to the member
        new_role: MemberRole,
        member: AccountRef,
    },
    /// An event indicating a member entered the chat by link. Can appear only inside groups
    EnterEvent(AccountRef),
    /// An event indicating a member left the chat. Can appear only inside groups
    LeaveEvent(AccountRef),
    /// An event indicating the chat was renamed. Can appear only inside groups
    RenameEvent {
        /// The new name of the chat
        new_name: String,
        member: AccountRef,
    },
    /// An event indicating the chat icon was changed. Can appear only inside groups
    ChangeIconEvent {
        new_icon_id: u64,
        member: AccountRef,
    },
    /// An event indicating the chat background was changed or removed. Can appear only inside
    /// groups
    ChangeBackgroundEvent {
        /// The ID of the new background image, or `None` if it was removed
        new_background_id: Option<u64>,
        member: AccountRef,
    },
    /// An event indicating chat parameters were changed. Can appear only inside groups
    ChangeParamsEvent { member: AccountRef },
    /// An unknown content type
    Unknown(i64),
    /// An unknown event type
    UnknownEvent(i64),
}
impl Content {
    /// The maximum allowed size in bytes for a static chat message image.
    pub const IMAGE_MAX_SIZE: usize = 256 * 1024;
    /// The maximum allowed dimension (width or height) for a static chat message image.
    pub const IMAGE_MAX_DIMENSION: usize = 1080;
    /// The maximum allowed size in bytes for a GIF chat message.
    pub const GIF_MAX_SIZE: usize = 1024 * 1024;
    /// The maximum allowed dimension (width or height) for a GIF chat message.
    pub const GIF_MAX_DIMENSION: usize = 400;
    /// The maximum number of static images that can be included in a single chat message.
    pub const IMAGES_MAX_COUNT: usize = 5;
    /// The maximum allowed duration for a voice message.
    pub const VOICE_MAX_DURATION: Duration = Duration::from_secs(20);

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
