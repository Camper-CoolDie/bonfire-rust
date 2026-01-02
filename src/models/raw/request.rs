use chrono::DateTime;
use serde::de::Error;
use serde::{Deserialize, Serialize};

use crate::models::{RootError, UnavailableError};
use crate::Result;

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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Request<'a, R> {
    #[serde(flatten)]
    pub content: R,
    #[serde(rename = "J_REQUEST_NAME")]
    pub request_name: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data_output: Vec<Option<u32>>,
    #[serde(rename = "J_API_ACCESS_TOKEN", skip_serializing_if = "Option::is_none")]
    pub api_access_token: Option<&'a str>,
    #[serde(rename = "J_API_BOT_TOKEN", skip_serializing_if = "Option::is_none")]
    pub api_bot_token: Option<&'a str>,
    #[serde(rename = "requestApiVersion")]
    pub api_version: &'static str,
}

#[derive(Deserialize)]
#[serde(tag = "J_STATUS", content = "J_RESPONSE")]
pub(crate) enum RootResponse<S> {
    #[serde(rename = "J_STATUS_OK")]
    Ok(S),
    #[serde(rename = "J_STATUS_ERROR")]
    Error(RawRootError),
}
