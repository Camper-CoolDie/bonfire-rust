use std::fmt;

use thiserror::Error;

use crate::client::RequestError;
use crate::{MeliorError, Result};

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

/// Represents errors that can occur during the login process.
#[derive(Error, Debug)]
pub enum LoginError {
    /// The provided email address is invalid
    #[error("invalid email")]
    InvalidEmail,
    /// The provided email address is not registered
    #[error("wrong email")]
    WrongEmail,
    /// The provided password is incorrect
    #[error("wrong password")]
    WrongPassword,
    /// The account attempting to log in is permanently banned
    #[error("account is hard banned")]
    HardBanned,
    /// Two-Factor Authentication (TFA) is required to continue logging in
    #[error("TFA is required to continue logging in ({0})")]
    TfaRequired(TfaRequired),
}

impl RequestError for LoginError {
    type Source = MeliorError;

    fn try_convert(error: &MeliorError) -> Result<Option<Self>> {
        Ok(match error.message.split_once(':') {
            Some(("InvalidEmail", _)) => Some(LoginError::InvalidEmail),
            Some(("WrongEmail", _)) => Some(LoginError::WrongEmail),
            Some(("WrongPassword", _)) => Some(LoginError::WrongPassword),
            Some(("HardBanned", _)) => Some(LoginError::HardBanned),
            _ => None,
        })
    }
}
