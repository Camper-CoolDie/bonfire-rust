use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

/// Represents an error that can occur while setting an account's age.
#[derive(Error, Debug)]
pub enum SetAgeError {
    /// The provided age is outside the allowed range
    #[error("invalid age")]
    InvalidAge,
}

impl RequestError for SetAgeError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_AGE" => Some(SetAgeError::InvalidAge),
            _ => None,
        })
    }
}
