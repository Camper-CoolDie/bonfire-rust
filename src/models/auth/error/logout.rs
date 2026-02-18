use thiserror::Error;

use crate::client::RequestError;
use crate::{MeliorError, Result};

/// Represents errors that can occur during the logout process.
#[derive(Error, Debug)]
pub enum LogoutError {
    /// The account attempting to log out is permanently banned
    #[error("account is hard banned")]
    HardBanned,
}

impl RequestError for LogoutError {
    fn try_from_melior(error: &MeliorError) -> Result<Option<Self>> {
        Ok(match error.message.split_once(':') {
            Some(("HardBanned", _)) => Some(LogoutError::HardBanned),
            _ => None,
        })
    }
}
