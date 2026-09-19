use serde::Deserialize;

use crate::models::notification::PublicationBlocked;
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::requests::raw::publication::RawKind;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawPublicationBlockedAfterReport {
    #[serde(rename = "blockUnitType")]
    pub publication_kind: RawKind,
    pub moderation_id: u64,
    #[serde(rename = "blockLastUnits")]
    pub with_last_publications: bool,
    #[serde(rename = "blockAccountDate")]
    pub banned_until: i64,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawPublicationBlockedAfterReport> for PublicationBlocked {
    type Error = Error;

    fn try_from(value: RawPublicationBlockedAfterReport) -> Result<Self> {
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
