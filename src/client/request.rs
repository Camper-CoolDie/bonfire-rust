use chrono::{DateTime, Utc};
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use thiserror::Error;

/// Represents an error from the root server. Some most common ones are split into predefined
/// variants. [RootServerError::Other] is used for errors that aren't predefined.
#[derive(Error, Clone, Debug, Deserialize)]
#[serde(tag = "code")]
pub enum RootError {
    /// You don't have enough permission. In very rare cases the server may also say why the access
    /// was denied
    #[error(
        "access denied{}",
        .message.as_ref().map(|message| format!(": {}", message)).unwrap_or_default())
    ]
    #[serde(rename = "ERROR_ACCESS")]
    AccessDenied {
        /// The reason of denying the access
        #[serde(
            rename = "messageError",
            deserialize_with = "crate::models::deserialize_string_or_none"
        )]
        message: Option<String>,
    },
    /// Something already exists (e.g. you already reacted)
    #[error("resource already exists")]
    #[serde(rename = "ERROR_ALREADY")]
    AlreadyExists,
    /// Your moderation reason is too long/too short or it contains profanity
    #[error("bad reason")]
    #[serde(rename = "ERROR_BAD_COMMENT")]
    BadReason,
    /// You're banned
    #[error("account is banned until {until}")]
    #[serde(rename = "ERROR_ACCOUNT_IS_BANED")]
    Banned {
        /// The date when your ban ends
        #[serde(
            rename = "params",
            deserialize_with = "RootError::deserialize_banned_until"
        )]
        until: DateTime<Utc>,
    },
    /// Something you're looking for is not available
    #[error("resource is unavailable")]
    #[serde(rename = "ERROR_GONE")]
    Unavailable(#[from] UnavailableError),
    /// Some other request-specific error
    #[error(
        "unknown error: {code}{}",
        .message.as_ref().map(|message| format!(" ({})", message)).unwrap_or_default())
    ]
    #[serde(untagged)]
    Other {
        /// Code of the error
        code: String,
        /// Message of the error
        #[serde(
            rename = "messageError",
            deserialize_with = "crate::models::deserialize_string_or_none"
        )]
        message: Option<String>,
        /// Parameters of the error
        params: Vec<String>,
    },
}
impl RootError {
    fn deserialize_banned_until<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<DateTime<Utc>, D::Error> {
        let params = <[String; 1]>::deserialize(deserializer)?;
        let millis = params[0].parse::<i64>().map_err(|error| {
            D::Error::custom(format!(
                "failed to convert banned_until into an integer: {error}"
            ))
        })?;
        DateTime::from_timestamp_millis(millis)
            .ok_or_else(|| D::Error::custom("timestamp is out of range"))
    }
}

/// Represents a specific type of
/// [client::RootServerError::Unavailable][RootServerError::Unavailable].
#[derive(Error, Clone, Debug, Deserialize)]
#[serde(tag = "messageError")]
pub enum UnavailableError {
    /// The publication was blocked by a moderator
    #[error("resource was blocked by moderators (moderation ID: {moderation_id})")]
    #[serde(rename = "GONE_BLOCKED")]
    Blocked {
        /// The ID of a moderation which contains the reason of blocking
        #[serde(
            rename = "params",
            deserialize_with = "UnavailableError::deserialize_moderation_id"
        )]
        moderation_id: i64,
    },
    /// Something you're searching for can't be found
    #[error("resource was not found")]
    #[serde(rename = "")]
    NotFound,
    /// The publication was removed by author
    #[error("resource was removed by author")]
    #[serde(rename = "REMOVE")]
    Removed,
    /// Some other request-specific error
    #[error("unknown unavailable error: {message}")]
    #[serde(untagged)]
    Other {
        /// Message of the error
        #[serde(rename = "messageError")]
        message: String,
        /// Parameters of the error
        params: Vec<String>,
    },
}
impl UnavailableError {
    fn deserialize_moderation_id<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<i64, D::Error> {
        let params = <[String; 1]>::deserialize(deserializer)?;
        params[0].parse::<i64>().map_err(|error| {
            D::Error::custom(format!(
                "failed to convert moderation_id into an integer: {error}"
            ))
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
pub(super) enum RootResponse<S> {
    #[serde(rename = "J_STATUS_OK")]
    Ok(S),
    #[serde(rename = "J_STATUS_ERROR")]
    Error(RootError),
}
