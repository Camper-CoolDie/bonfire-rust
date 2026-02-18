use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

/// Represents an error that can occur while setting an account's status or description.
#[derive(Error, Debug)]
pub enum SetProfileTextError {
    /// The provided text exceeds the maximum allowed length
    #[error("text is too long")]
    TooLong,
}

impl RequestError for SetProfileTextError {
    fn try_from_root(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_SIZE" => {
                Some(SetProfileTextError::TooLong)
            }
            _ => None,
        })
    }
}
