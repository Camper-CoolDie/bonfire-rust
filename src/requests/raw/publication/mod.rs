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
pub(crate) use status::RawStatus;

use crate::models::publication::{Publishable, Status};
use crate::models::{Account, Fandom, Publication};
use crate::requests::raw::{RawAccount, RawCategory, RawFandom};
use crate::{Error, Result};

pub(crate) trait RawPublishable: Sized {
    type Target: Publishable;

    fn new(data: Value, kind: RawKind) -> Result<Self>;
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawPublication<T: RawPublishable = AnyRawPublication> {
    pub id: u64,
    pub fandom: RawFandom,
    #[serde(rename = "creator")]
    pub author: RawAccount,
    pub category: RawCategory,
    #[serde(rename = "dateCreate")]
    pub created_at: i64,
    #[serde(rename = "unitType")]
    pub kind: RawKind,
    #[serde(rename = "parentUnitId")]
    pub parent_id: u64,
    #[serde(rename = "parentUnitType")]
    pub parent_kind: RawKind,
    #[serde(rename = "karmaCount")]
    pub karma: f64,
    pub my_karma: f64,
    pub status: RawStatus,
    #[serde(rename = "closed")]
    pub is_closed: bool,
    #[serde(rename = "subUnitsCount")]
    pub comments_count: u64,
    #[serde(rename = "important")]
    pub importance: i64,
    #[serde(rename = "blacklisted")]
    pub is_hidden: bool,
    #[serde(rename = "nsfw")]
    pub is_nsfw: bool,
    pub hotness: f32,
    #[serde(flatten)]
    pub additional_data: Value,
    #[serde(skip)]
    pub _marker: PhantomData<T>,
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
            status: Option::<Status>::try_from(value.status)?,
            is_closed: value.is_closed,
            comments_count: value.comments_count,
            is_important: matches!(value.importance, -1),
            is_hidden: value.is_hidden,
            is_nsfw: value.is_nsfw,
            hotness: value.hotness,
        })
    }
}
