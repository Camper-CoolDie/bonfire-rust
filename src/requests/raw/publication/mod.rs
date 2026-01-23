mod kind;
mod reaction;

use std::marker::PhantomData;

use chrono::DateTime;
pub(crate) use kind::*;
pub(crate) use reaction::RawReaction;
use serde::de::Error as _;
use serde::Deserialize;
use serde_json::Value;
use serde_repr::Deserialize_repr;

use crate::models::publication::{PublicationInheritor, PublicationKind, PublicationStatus};
use crate::models::{Account, Fandom, Publication};
use crate::requests::raw::{RawAccount, RawCategory, RawFandom};
use crate::{Error, Result};

pub(crate) trait RawPublicationInheritor: Sized {
    type Target: PublicationInheritor;

    fn new(data: Value, id: u64, kind: RawPublicationKind) -> Result<Self>;
}

#[derive(Deserialize_repr)]
#[repr(i64)]
pub(crate) enum RawPublicationStatus {
    Unspecified = 0,
    Draft = 1,
    Published = 2,
    Blocked = 3,
    DeepBlocked = 4,
    Pending = 5,
}

impl From<RawPublicationStatus> for PublicationStatus {
    fn from(value: RawPublicationStatus) -> Self {
        match value {
            RawPublicationStatus::Unspecified => PublicationStatus::Unspecified,
            RawPublicationStatus::Draft => PublicationStatus::Draft,
            RawPublicationStatus::Published => PublicationStatus::Published,
            RawPublicationStatus::Blocked => PublicationStatus::Blocked,
            RawPublicationStatus::DeepBlocked => PublicationStatus::DeepBlocked,
            RawPublicationStatus::Pending => PublicationStatus::Pending,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawPublication<T: RawPublicationInheritor = AnyRawPublication> {
    pub id: u64,
    fandom: RawFandom,
    #[serde(rename = "creator")]
    author: RawAccount,
    category: RawCategory,
    #[serde(rename = "dateCreate")]
    created_at: i64,
    #[serde(rename = "unitType")]
    kind: RawPublicationKind,
    #[serde(rename = "parentUnitId")]
    parent_id: u64,
    #[serde(rename = "parentUnitType")]
    parent_kind: i64,
    #[serde(rename = "karmaCount")]
    karma: f64,
    my_karma: f64,
    status: RawPublicationStatus,
    #[serde(rename = "closed")]
    is_closed: bool,
    #[serde(rename = "subUnitsCount")]
    comments_count: u64,
    #[serde(rename = "important")]
    importance: i64,
    #[serde(rename = "blacklisted")]
    is_hidden: bool,
    #[serde(rename = "nsfw")]
    is_nsfw: bool,
    hotness: f32,
    #[serde(flatten)]
    additional_data: Value,
    #[serde(skip)]
    marker: PhantomData<T>,
    // TODO: tag_1, tag_2, tag_s_1, etc.
}

impl<T: RawPublicationInheritor> TryFrom<RawPublication<T>> for Publication<T::Target>
where
    T::Target: TryFrom<T>,
    Error: From<<T::Target as TryFrom<T>>::Error>,
{
    type Error = Error;

    fn try_from(value: RawPublication<T>) -> Result<Self> {
        let parent_kind = match value.parent_kind {
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
        };

        Ok(Self {
            kind: T::new(value.additional_data, value.id, value.kind)?.try_into()?,
            id: value.id,
            // For some mysterious reason .try_into() doesn't work here
            fandom: Fandom::try_from(value.fandom)?,
            author: Account::try_from(value.author)?,
            category: value.category.into(),
            created_at: DateTime::from_timestamp_millis(value.created_at).ok_or_else(|| {
                serde_json::Error::custom(format!("timestamp {} is out of range", value.created_at))
            })?,
            parent_id: match value.parent_id {
                0 => None,
                id => Some(id),
            },
            parent_kind,
            karma: value.karma / 100.,
            my_karma: match value.my_karma {
                0. => None,
                karma => Some(karma / 100.),
            },
            status: value.status.into(),
            is_closed: value.is_closed,
            comments_count: value.comments_count,
            is_important: matches!(value.importance, -1),
            is_hidden: value.is_hidden,
            is_nsfw: value.is_nsfw,
            hotness: value.hotness,
        })
    }
}
