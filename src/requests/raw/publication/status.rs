use serde::de::Error as _;

use crate::models::publication::PublicationStatus;
use crate::{Error, Result};

pub(crate) enum RawPublicationStatus {
    Unknown(i64),
    Draft,
    Published,
    Blocked,
    DeepBlocked,
    Pending,
}

impl From<i64> for RawPublicationStatus {
    fn from(value: i64) -> Self {
        match value {
            1 => RawPublicationStatus::Draft,
            2 => RawPublicationStatus::Published,
            3 => RawPublicationStatus::Blocked,
            4 => RawPublicationStatus::DeepBlocked,
            5 => RawPublicationStatus::Pending,
            other => RawPublicationStatus::Unknown(other),
        }
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
            RawPublicationStatus::Unknown(unknown) => Err(serde_json::Error::custom(format!(
                "invalid value: {}, expected one of: {}",
                unknown,
                (0..=5)
                    .map(|n| n.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )))?,
        })
    }
}
