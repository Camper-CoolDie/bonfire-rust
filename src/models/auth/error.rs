use std::fmt;

use thiserror::Error;

/// Represents the type of a Two-Factor Authentication (TFA) session.
#[derive(Debug)]
pub enum TfaKind {
    /// Log in using a Time-based One-Time Password (TOTP)
    Totp,
    /// Log in after verifying the attempt through a link sent to the account owner's email
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

/// Represents data required to continue logging in using Two-Factor Authentication (TFA).
#[derive(Debug)]
pub struct TfaRequired {
    /// The type of this TFA session
    pub kind: TfaKind,
    /// A wait token specific to this TFA session
    pub wait_token: String,
}

impl fmt::Display for TfaRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

/// Represents errors that can occur during authentication operations.
///
/// # Source
///
/// An `Error` can arise from an unexpected server response or an unauthenticated client state.
#[derive(Error, Debug)]
pub enum Error {
    /// The client is already authenticated
    #[error("authenticated client")]
    AlreadyAuthenticated,
    /// Two-Factor Authentication (TFA) is required to continue logging in
    #[error("TFA is required to continue logging in ({0})")]
    TfaRequired(TfaRequired),
    /// The client is unauthenticated
    #[error("unauthenticated client")]
    Unauthenticated,
}
