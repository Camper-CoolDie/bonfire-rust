use chrono::{DateTime, Utc};
use thiserror::Error;

/// Represents an error from the root server. Some most common ones are split into predefined
/// variants. [`RootError::Other`] is used for errors that aren't predefined.
#[derive(Error, Debug)]
pub enum RootError {
    /// You don't have enough permission. In very rare cases the server may also say why the access
    /// was denied
    #[error(
        "access denied{}",
        .message.as_ref().map(|message| format!(": {message}")).unwrap_or_default())
    ]
    AccessDenied {
        /// The reason of denying the access
        message: Option<String>,
    },
    /// Something already exists (e.g. you already reacted)
    #[error("resource already exists")]
    AlreadyExists,
    /// Your moderation reason is too long/too short or it contains profanity
    #[error("bad reason")]
    BadReason,
    /// You're banned
    #[error("account is banned until {until}")]
    Banned {
        /// The date when your ban ends
        until: DateTime<Utc>,
    },
    /// Something you're looking for is not available
    #[error("resource is unavailable")]
    Unavailable(#[from] UnavailableError),
    /// Some other request-specific error
    #[error(
        "unknown error: {code}{}",
        .message.as_ref().map(|message| format!(" ({message})")).unwrap_or_default())
    ]
    Other {
        /// Code of the error
        code: String,
        /// Message of the error
        message: Option<String>,
        /// Parameters of the error
        params: Vec<String>,
    },
}

/// Represents a specific type of [`RootError::Unavailable`].
#[derive(Error, Debug)]
pub enum UnavailableError {
    /// The publication was blocked by a moderator
    #[error("resource was blocked by moderators (moderation ID: {moderation_id})")]
    Blocked {
        /// The ID of a moderation which contains the reason of blocking
        moderation_id: i64,
    },
    /// Something you're searching for can't be found
    #[error("resource was not found")]
    NotFound,
    /// The publication was removed by author
    #[error("resource was removed by author")]
    Removed,
    /// Some other request-specific error
    #[error("unknown unavailable error: {message}")]
    Other {
        /// Message of the error
        message: String,
        /// Parameters of the error
        params: Vec<String>,
    },
}
