use std::fmt;

use thiserror::Error;

/// Represents a type of the TFA.
#[derive(Debug)]
pub enum TfaKind {
    /// Log in again using a TOTP (Time-based One Time Password)
    Totp,
    /// Log in after an owner of the account verified the login through a link sent to their email
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
    /// A type of the TFA
    pub kind: TfaKind,
    /// A wait token of the TFA
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
/// An `auth::Error` can be the result of a non-standart response or an unauthenticated client.
#[derive(Error, Debug)]
pub enum Error {
    /// Client is already authenticated
    #[error("authenticated client")]
    AlreadyAuthenticated,
    /// TFA is required to continue logging in
    #[error("TFA is required to continue logging in ({0})")]
    TfaRequired(TfaRequired),
    /// Client is unauthenticated
    #[error("unauthenticated client")]
    Unauthenticated,
}
