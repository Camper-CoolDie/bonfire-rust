mod kind;
mod tag;

use std::marker::PhantomData;

use chrono::DateTime;
pub(crate) use kind::*;
use serde::Deserialize;
use serde::de::Error as _;
use serde_json::Value;
pub(crate) use tag::RawTag;

use crate::models::chat::Messageable;
use crate::models::{Chat, Publication};
use crate::requests::raw::{RawChatMessage, RawPublication};
use crate::{Error, Result};

pub(crate) trait RawMessageable: Sized {
    type Target: Messageable;

    fn new(data: Value, tag: RawTag) -> Result<Self>;
}

#[derive(Deserialize)]
#[serde(rename = "camelCase")]
pub(crate) struct RawChat<T: RawMessageable = AnyRawChat> {
    pub tag: RawTag,
    #[serde(rename = "unitChatMessage")]
    pub last_message: RawPublication<RawChatMessage>,
    pub unread_count: u64,
    #[serde(rename = "readDate")]
    pub read_at: i64,
    #[serde(flatten)]
    pub additional_data: Value,
    #[serde(skip)]
    pub _marker: PhantomData<T>,
}

impl<T: RawMessageable> TryFrom<RawChat<T>> for Chat<T::Target>
where
    T::Target: TryFrom<T>,
    Error: From<<T::Target as TryFrom<T>>::Error>,
{
    type Error = Error;

    fn try_from(value: RawChat<T>) -> Result<Self> {
        Ok(Self {
            kind: T::new(value.additional_data, value.tag)?.try_into()?,
            last_message: Option::<Publication<_>>::try_from(value.last_message)?,
            unread_count: value.unread_count,
            read_at: match value.read_at {
                0 => None,
                timestamp => Some(DateTime::from_timestamp_millis(timestamp).ok_or_else(|| {
                    serde_json::Error::custom(format!("timestamp {timestamp} is out of range"))
                })?),
            },
        })
    }
}
