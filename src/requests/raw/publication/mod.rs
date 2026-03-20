mod kind;
mod reaction;
mod status;

use std::marker::PhantomData;

use chrono::DateTime;
pub(crate) use kind::*;
pub(crate) use reaction::RawReaction;
use serde::Deserialize;
use serde::de::Error as _;
use serde_json::Value;
pub(crate) use status::RawPublicationStatus;

use crate::models::publication::{PublicationStatus, Publishable};
use crate::models::{Account, Fandom, Publication};
use crate::requests::raw::{RawAccount, RawCategory, RawFandom};
use crate::{Error, Result};

pub(crate) trait RawPublishable: Sized {
    type Target: Publishable;

    fn new(data: Value, kind: RawPublicationKind) -> Result<Self>;
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawPublication<T: RawPublishable = AnyRawPublication> {
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
    parent_kind: RawPublicationKind,
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
    _marker: PhantomData<T>,
    // TODO: tag_1, tag_2, tag_s_1, etc.
}

impl<T: RawPublishable> TryFrom<RawPublication<T>> for Publication<T::Target>
where
    T::Target: TryFrom<T>,
    Error: From<<T::Target as TryFrom<T>>::Error>,
{
    type Error = Error;

    fn try_from(value: RawPublication<T>) -> Result<Self> {
        // For some mysterious reason .try_into() doesn't work here for non-T types
        Ok(Self {
            kind: T::new(value.additional_data, value.kind)?.try_into()?,
            id: value.id,
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
            parent_kind: value.parent_kind.into(),
            karma: value.karma / 100.0,
            my_karma: match value.my_karma {
                0.0 => None,
                karma => Some(karma / 100.0),
            },
            status: Option::<PublicationStatus>::try_from(value.status)?,
            is_closed: value.is_closed,
            comments_count: value.comments_count,
            is_important: matches!(value.importance, -1),
            is_hidden: value.is_hidden,
            is_nsfw: value.is_nsfw,
            hotness: value.hotness,
        })
    }
}
