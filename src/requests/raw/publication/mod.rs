mod kind;
mod reaction;
mod status;

use std::marker::PhantomData;

pub(crate) use kind::*;
pub(crate) use reaction::RawReaction;
use serde::Deserialize;
use serde_json::Value;
pub(crate) use status::RawStatus;

use crate::models::Publication;
use crate::models::publication::{Publishable, Status};
use crate::requests::raw::timestamp_from_millis;
use crate::{Error, Result};

pub(crate) trait RawPublishable: Sized {
    type Target: Publishable;

    fn new(data: Value, kind: RawKind) -> Result<Self>;
}

#[derive(Deserialize)]
pub(crate) struct RawPublication<T: RawPublishable = AnyRawPublication> {
    pub id: u64,
    #[serde(rename = "dateCreate")]
    pub created_at: i64,
    #[serde(rename = "unitType")]
    pub kind: RawKind,
    pub status: RawStatus,
    pub hotness: f32,
    #[serde(flatten)]
    pub additional_data: Value,
    #[serde(skip)]
    pub _marker: PhantomData<T>,
}

impl<T: RawPublishable> TryFrom<RawPublication<T>> for Publication<T::Target>
where
    T::Target: TryFrom<T>,
    Error: From<<T::Target as TryFrom<T>>::Error>,
{
    type Error = Error;

    fn try_from(value: RawPublication<T>) -> Result<Self> {
        Ok(Self {
            kind: T::new(value.additional_data, value.kind)?.try_into()?,
            id: value.id,
            created_at: timestamp_from_millis(value.created_at)?,
            status: Option::<Status>::try_from(value.status)?,
            hotness: value.hotness,
        })
    }
}

impl<T: RawPublishable> TryFrom<RawPublication<T>> for Option<Publication<T::Target>>
where
    T::Target: TryFrom<T>,
    Error: From<<T::Target as TryFrom<T>>::Error>,
{
    type Error = Error;

    fn try_from(value: RawPublication<T>) -> Result<Self> {
        // Sometimes, the publication ID is non-zero, but its status is (which will cause an
        // UnknownVariant error later on)
        Ok(match value.status {
            RawStatus::Unknown(0) => None,
            _ => Some(Publication::<_>::try_from(value)?),
        })
    }
}
