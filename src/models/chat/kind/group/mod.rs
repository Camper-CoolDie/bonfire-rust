mod role;
mod status;

use chrono::{DateTime, Utc};
pub use role::MemberRole;
pub use status::MemberStatus;

use crate::models::ImageRef;
use crate::models::chat::{Messageable, Tag};
use crate::sealed::Sealed;

/// Represents a group chat.
#[derive(Default, Clone, Debug)]
pub struct Group {
    /// The unique identifier of the group chat
    pub id: u64,
    /// The name of the group chat
    pub name: String,
    /// The icon image of the group chat
    pub icon: ImageRef,
    /// The background image of the group chat, if set
    pub background: Option<ImageRef>,
    /// The status of the authenticated user in this group chat
    pub my_status: MemberStatus,
    /// Indicates if the authenticated user is subscribed to this group chat
    pub is_subscribed: bool,
    /// The number of subscribers to this group chat
    pub subscribers_count: u64,
    /// The timestamp when the authenticated user left this group chat, if applicable
    pub left_at: Option<DateTime<Utc>>,
    /// Indicates if this group chat is public (accessible via link)
    pub is_public: bool,
    /// Indicates if invites are allowed in this group chat
    pub allow_invites: bool,
    /// Indicates if only admins can change chat parameters
    pub allow_changes: bool,
}
impl Group {
    /// Creates a new `Group` instance with the given `id`.
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }
}

impl Messageable for Group {
    /// Returns the chat's tag as [`ChatTag::Group`][Tag::Group].
    fn tag(&self) -> Tag {
        Tag::Group { id: self.id }
    }
}

impl Sealed for Group {}
