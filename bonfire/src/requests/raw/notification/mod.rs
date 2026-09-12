mod kind;

pub(crate) use kind::*;
use serde::Deserialize;
use serde_json::Value;

use crate::models::Notification;
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawNotification {
    #[serde(rename = "J_N_DATE_CREATE")]
    pub sent_at: i64,
    #[serde(rename = "J_N_TYPE")]
    pub kind: RawKind,
    #[serde(flatten)]
    pub additional_data: Value,
}

impl TryFrom<RawNotification> for Notification {
    type Error = Error;

    fn try_from(value: RawNotification) -> Result<Self> {
        Ok(Self {
            content: AnyRawNotification::new(value.additional_data, value.kind)?.try_into()?,
            sent_at: timestamp_from_millis(value.sent_at)?,
        })
    }
}
