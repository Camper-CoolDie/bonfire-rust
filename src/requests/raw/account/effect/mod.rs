mod kind;
mod origin;
mod reason_kind;

use chrono::DateTime;
pub(crate) use kind::RawKind;
use origin::IntoOriginOptions;
pub(crate) use reason_kind::RawReasonKind;
use serde::Deserialize;
use serde::de::Error as _;

use crate::models::Effect;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawEffect {
    pub id: u64,
    pub account_id: u64,
    #[serde(rename = "dateCreate")]
    pub applied_at: i64,
    #[serde(rename = "dateEnd")]
    pub ends_at: i64,
    #[serde(rename = "effectIndex")]
    pub kind: RawKind,
    #[serde(rename = "tag")]
    pub is_system: i64,
    #[serde(rename = "comment")]
    pub reason: String,
    #[serde(rename = "commentTag")]
    pub reason_kind: RawReasonKind,
    pub from_account_name: String,
}

impl TryFrom<RawEffect> for Effect {
    type Error = Error;

    fn try_from(value: RawEffect) -> Result<Self> {
        let is_system = value.is_system == 1;

        Ok(Self {
            id: value.id,
            account_id: value.account_id,
            applied_at: DateTime::from_timestamp_millis(value.applied_at).ok_or_else(|| {
                serde_json::Error::custom(format!("timestamp {} is out of range", value.applied_at))
            })?,
            ends_at: DateTime::from_timestamp_millis(value.ends_at).ok_or_else(|| {
                serde_json::Error::custom(format!("timestamp {} is out of range", value.ends_at))
            })?,
            kind: value.kind.try_into()?,
            origin: IntoOriginOptions {
                is_system,
                reason: value.reason,
                reason_kind: value.reason_kind,
                from_account_name: value.from_account_name,
            }
            .try_into()?,
        })
    }
}
