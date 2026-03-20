use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::publication::PublicationStatus;
use crate::{Error, Result};

pub(crate) enum RawPublicationStatus {
    Draft,
    Published,
    Blocked,
    DeepBlocked,
    Pending,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawPublicationStatus {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawPublicationStatus::Draft,
            2 => RawPublicationStatus::Published,
            3 => RawPublicationStatus::Blocked,
            4 => RawPublicationStatus::DeepBlocked,
            5 => RawPublicationStatus::Pending,
            other => RawPublicationStatus::Unknown(other),
        })
    }
}

impl TryFrom<RawPublicationStatus> for Option<PublicationStatus> {
    type Error = Error;

    fn try_from(value: RawPublicationStatus) -> Result<Self> {
        Ok(match value {
            RawPublicationStatus::Unknown(0) => None,
            RawPublicationStatus::Draft => Some(PublicationStatus::Draft),
            RawPublicationStatus::Published => Some(PublicationStatus::Published),
            RawPublicationStatus::Blocked => Some(PublicationStatus::Blocked),
            RawPublicationStatus::DeepBlocked => Some(PublicationStatus::DeepBlocked),
            RawPublicationStatus::Pending => Some(PublicationStatus::Pending),
            RawPublicationStatus::Unknown(unknown) => Err(Error::UnknownVariant(unknown))?,
        })
    }
}
