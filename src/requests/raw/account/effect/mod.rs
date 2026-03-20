mod kind;
mod reason_kind;

use chrono::DateTime;
pub(crate) use kind::RawEffectKind;
pub(crate) use reason_kind::RawEffectReasonKind;
use serde::Deserialize;
use serde::de::Error as _;

use crate::models::Effect;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawEffect {
    pub id: u64,
    account_id: u64,
    #[serde(rename = "dateCreate")]
    applied_at: i64,
    #[serde(rename = "dateEnd")]
    ends_at: i64,
    #[serde(rename = "comment")]
    reason: String,
    #[serde(rename = "effectIndex")]
    kind: i64,
    #[serde(rename = "tag")]
    is_system: i64,
    #[serde(rename = "commentTag")]
    reason_kind: i64,
    from_account_name: String,
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
            reason: (!is_system).then_some(value.reason),
            kind: RawEffectKind::from(value.kind).try_into()?,
            is_system,
            reason_kind: RawEffectReasonKind::from(value.reason_kind).try_into()?,
            from_account_name: (!is_system).then_some(value.from_account_name),
        })
    }
}
