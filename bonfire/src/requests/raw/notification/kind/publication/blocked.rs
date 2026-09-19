use serde::Deserialize;

use crate::models::notification::PublicationBlocked;
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::requests::raw::publication::RawKind;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawPublicationBlocked {
    #[serde(rename = "blockUnitType")]
    pub publication_kind: RawKind,
    #[serde(rename = "J_MODERATION_ID")]
    pub moderation_id: u64,
    #[serde(rename = "J_BLOCK_LAST")]
    pub with_last_publications: bool,
    #[serde(rename = "J_BLOCK_ACCOUNT_DATE")]
    pub banned_until: i64,
    #[serde(rename = "J_COMMENT")]
    pub reason: String,
}

impl TryFrom<RawPublicationBlocked> for PublicationBlocked {
    type Error = Error;

    fn try_from(value: RawPublicationBlocked) -> Result<Self> {
        Ok(Self {
            publication_kind: value.publication_kind.into(),
            moderation_id: value.moderation_id,
            with_last_publications: value.with_last_publications,
            is_punished: value.banned_until != 0,
            banned_until: match value.banned_until {
                // -1: warn, 0: do nothing
                -1 | 0 => None,
                timestamp => Some(timestamp_from_millis(timestamp)?),
            },
            reason: value.reason,
        })
    }
}
