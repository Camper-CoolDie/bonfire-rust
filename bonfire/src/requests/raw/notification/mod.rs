mod kind;

pub(crate) use kind::*;
use serde::Deserialize;

use crate::models::Notification;
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawNotification {
    #[serde(flatten)]
    pub content: AnyRawNotification,
    #[serde(rename = "J_N_DATE_CREATE")]
    pub sent_at: i64,
}

impl TryFrom<RawNotification> for Notification {
    type Error = Error;

    fn try_from(value: RawNotification) -> Result<Self> {
        Ok(Self {
            content: value.content.try_into()?,
            sent_at: timestamp_from_millis(value.sent_at)?,
        })
    }
}
