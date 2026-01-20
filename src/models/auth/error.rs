use std::fmt;

use thiserror::Error;

/// Represents a type of a TFA session.
#[derive(Debug)]
pub enum TfaKind {
    /// Log in again using a TOTP (Time-based One Time Password)
    Totp,
    /// Log in after an owner of the account verified this log-in attempt through a link sent to
    /// their email
    EmailLink,
}

impl fmt::Display for TfaKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Totp => write!(f, "TOTP"),
            Self::EmailLink => write!(f, "email link"),
        }
    }
}

/// Represents data to continue logging in using TFA (Two-Factor Authentication).
#[derive(Debug)]
pub struct TfaRequired {
    /// A type of this TFA session
    pub kind: TfaKind,
    /// A wait token for this TFA session
    pub wait_token: String,
}

impl fmt::Display for TfaRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

/// Represents errors that can occur while authenticating.
///
/// # Source
///
/// An `AuthError` can be the result of a non-standart response or an unauthenticated client.
#[derive(Error, Debug)]
pub enum Error {
    /// The client is already authenticated
    #[error("authenticated client")]
    AlreadyAuthenticated,
    /// TFA is required to continue logging in
    #[error("TFA is required to continue logging in ({0})")]
    TfaRequired(TfaRequired),
    /// The client is unauthenticated
    #[error("unauthenticated client")]
    Unauthenticated,
}
