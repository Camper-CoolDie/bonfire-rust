use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

/// Represents errors that can occur while setting an account's name.
#[derive(Error, Debug)]
pub enum SetNameError {
    /// The account's name has already been set and can no longer be changed, unless it was cleared
    /// by administrators
    #[error("name is already set")]
    AlreadySet,
    /// The provided name contains invalid characters, or is too long/short
    #[error("invalid name")]
    InvalidName,
    /// The provided name is already in use by another account
    #[error("name is already taken")]
    Taken,
}

impl RequestError for SetNameError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_LOGIN_IS_NOT_DEFAULT" => {
                Some(SetNameError::AlreadySet)
            }
            RootError::Other { code, .. } if code == "E_LOGIN_CHARS" => {
                Some(SetNameError::InvalidName)
            }
            RootError::Other { code, .. } if code == "E_LOGIN_NOT_ENABLED" => {
                Some(SetNameError::Taken)
            }
            _ => None,
        })
    }
}
