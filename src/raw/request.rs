use chrono::DateTime;
use serde::de::Error;
use serde::Deserialize;

use crate::{Result, RootError, UnavailableError};

#[derive(Deserialize)]
#[serde(tag = "code")]
pub(crate) enum RawRootError {
    #[serde(rename = "ERROR_ACCESS")]
    AccessDenied {
        #[serde(rename = "messageError")]
        message: String,
    },
    #[serde(rename = "ERROR_ALREADY")]
    AlreadyExists,
    #[serde(rename = "ERROR_BAD_COMMENT")]
    BadReason,
    #[serde(rename = "ERROR_ACCOUNT_IS_BANED")]
    Banned { params: [String; 1] },
    #[serde(rename = "ERROR_GONE")]
    Unavailable(RawUnavailableError),
    #[serde(untagged)]
    Other {
        code: String,
        #[serde(rename = "messageError")]
        message: String,
        params: Vec<String>,
    },
}

impl TryFrom<RawRootError> for RootError {
    type Error = crate::Error;

    fn try_from(value: RawRootError) -> Result<Self> {
        Ok(match value {
            RawRootError::AccessDenied { message } => RootError::AccessDenied {
                message: match message.as_str() {
                    "" => None,
                    _ => Some(message),
                },
            },
            RawRootError::AlreadyExists => RootError::AlreadyExists,
            RawRootError::BadReason => RootError::BadReason,
            RawRootError::Banned { params } => {
                let millis = params[0].parse::<i64>().map_err(|error| {
                    serde_json::Error::custom(format!(
                        "failed to convert field `banned_until` into an integer: {error}"
                    ))
                })?;
                RootError::Banned {
                    until: {
                        DateTime::from_timestamp_millis(millis).ok_or_else(|| {
                            serde_json::Error::custom(format!(
                                "timestamp {} is out of range",
                                millis
                            ))
                        })?
                    },
                }
            }
            RawRootError::Unavailable(error) => RootError::Unavailable(error.try_into()?),
            RawRootError::Other {
                code,
                message,
                params,
            } => RootError::Other {
                code,
                message: match message.as_str() {
                    "" => None,
                    _ => Some(message),
                },
                params,
            },
        })
    }
}

#[derive(Deserialize)]
#[serde(tag = "messageError")]
pub(crate) enum RawUnavailableError {
    #[serde(rename = "GONE_BLOCKED")]
    Blocked {
        #[serde(rename = "params")]
        params: [String; 1],
    },
    #[serde(rename = "")]
    NotFound,
    #[serde(rename = "REMOVE")]
    Removed,
    #[serde(untagged)]
    Other {
        #[serde(rename = "messageError")]
        message: String,
        params: Vec<String>,
    },
}

impl TryFrom<RawUnavailableError> for UnavailableError {
    type Error = crate::Error;

    fn try_from(value: RawUnavailableError) -> Result<Self> {
        Ok(match value {
            RawUnavailableError::Blocked { params } => {
                let moderation_id = params[0].parse::<i64>().map_err(|error| {
                    serde_json::Error::custom(format!(
                        "failed to convert field `moderation_id` into an integer: {error}"
                    ))
                })?;
                UnavailableError::Blocked { moderation_id }
            }
            RawUnavailableError::NotFound => UnavailableError::NotFound,
            RawUnavailableError::Removed => UnavailableError::Removed,
            RawUnavailableError::Other { message, params } => {
                UnavailableError::Other { message, params }
            }
        })
    }
}
