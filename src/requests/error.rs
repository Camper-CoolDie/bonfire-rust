use std::fmt::Debug;

use chrono::{DateTime, Utc};
use thiserror::Error;

use crate::client::RequestErrorSource;

/// Represents errors returned by the Root server.
///
/// Common errors are categorized into predefined variants, while [`RootError::Other`]
/// captures any non-predefined errors.
#[derive(Error, Debug)]
pub enum RootError {
    /// The request was denied due to insufficient permissions
    #[error(
        "access denied{}",
        .message.as_ref().map(|message| format!(": {message}")).unwrap_or_default())
    ]
    AccessDenied {
        /// The specific reason why access was denied
        message: Option<String>,
    },
    /// A resource already exists (e.g., attempting to react multiple times)
    #[error("resource already exists")]
    AlreadyExists,
    /// The provided moderation reason is invalid (e.g., too long, too short, or contains
    /// profanity)
    #[error("bad reason")]
    BadReason,
    /// The account is banned until a specified date and time
    #[error("account is banned until {until}")]
    Banned {
        /// The date and time when the ban on this account ends
        until: DateTime<Utc>,
    },
    /// The requested resource is unavailable
    #[error("resource is unavailable")]
    Unavailable(#[from] UnavailableError),
    /// An unknown error with a specific code and message
    #[error(
        "unknown error: {code}{}",
        .message.as_ref().map(|message| format!(" ({message})")).unwrap_or_default())
    ]
    Other {
        /// The error code
        code: String,
        /// A descriptive error message
        message: Option<String>,
        /// Additional parameters associated with the error
        params: Vec<String>,
    },
}

impl RequestErrorSource for RootError {}

/// Represents specific reasons why a resource might be unavailable, typically associated with a
/// [`RootError::Unavailable`].
#[derive(Error, Debug)]
pub enum UnavailableError {
    /// The publication was blocked by a moderator
    #[error("publication was blocked by a moderator (moderation ID: {moderation_id})")]
    Blocked {
        /// The ID of the moderation event that contains the reason for blocking
        moderation_id: i64,
    },
    /// The requested resource could not be found
    #[error("resource could not be found")]
    NotFound,
    /// The publication was removed by its author
    #[error("publication was removed by author")]
    Removed,
    /// An unknown unavailable error
    #[error("unknown unavailable error: {message}")]
    Other {
        /// A descriptive error message
        message: String,
        /// Additional parameters associated with the error
        params: Vec<String>,
    },
}
