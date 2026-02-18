use thiserror::Error;

use crate::client::RequestError;
use crate::{MeliorError, Result};

/// Represents errors that can occur during the token refresh process.
#[derive(Error, Debug)]
pub enum RefreshError {
    /// The refresh token has expired, requiring a new login
    #[error("refresh token has expired")]
    TokenExpired,
}

impl RequestError for RefreshError {
    fn try_from_melior(error: &MeliorError) -> Result<Option<Self>> {
        Ok(match error.message.split_once(':') {
            Some(("TokenExpired", _)) => Some(RefreshError::TokenExpired),
            _ => None,
        })
    }
}
