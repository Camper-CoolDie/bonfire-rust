mod post;

pub use post::Post;
use serde::{Deserialize, Deserializer, Serializer};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Represents a publication type.
#[derive(Default, Clone, Debug, Deserialize_repr, Serialize_repr)]
#[repr(i64)]
pub enum PublicationKind {
    /// The publication has an unknown or unspecified type
    #[default]
    #[serde(other)]
    Unknown = 20,
    /// The publication is a comment
    Comment = 1,
    /// The publication is a chat message
    ChatMessage = 8,
    /// The publication is a post
    Post = 9,
    /// The publication is a post tag
    PostTag = 10,
    /// The publication is a moderation
    Moderation = 11,
    /// The publication is a user event
    UserEvent = 12,
    /// The publication is a sticker pack
    StickerPack = 15,
    /// The publication is a sticker
    Sticker = 16,
    /// The publication is a moderation event
    ModerationEvent = 17,
    /// The publication is an administration event
    AdminEvent = 18,
    /// The publication is a fandom event
    FandomEvent = 19,
    /// The publication is a quest
    Quest = 21,
}
impl PublicationKind {
    pub(crate) fn serialize_or_none<S: Serializer>(
        value: &Option<PublicationKind>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(match value {
            None => 0,
            Some(value) => value.clone() as i64,
        })
    }

    pub(crate) fn deserialize_or_none<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<PublicationKind>, D::Error> {
        let value = i64::deserialize(deserializer)?;
        Ok(match value {
            0 => None,
            1 => Some(PublicationKind::Comment),
            8 => Some(PublicationKind::ChatMessage),
            9 => Some(PublicationKind::Post),
            10 => Some(PublicationKind::PostTag),
            11 => Some(PublicationKind::Moderation),
            12 => Some(PublicationKind::UserEvent),
            15 => Some(PublicationKind::StickerPack),
            16 => Some(PublicationKind::Sticker),
            17 => Some(PublicationKind::ModerationEvent),
            18 => Some(PublicationKind::AdminEvent),
            19 => Some(PublicationKind::FandomEvent),
            21 => Some(PublicationKind::Quest),
            _ => Some(PublicationKind::Unknown),
        })
    }
}
