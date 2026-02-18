use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

/// Represents errors that can occur while setting an account's referrer.
#[derive(Error, Debug)]
pub enum SetReferrerError {
    /// The referrer for this account has already been set.
    #[error("referrer already set")]
    AlreadySet,
}

impl RequestError for SetReferrerError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_ALREADY_SET" => {
                Some(SetReferrerError::AlreadySet)
            }
            _ => None,
        })
    }
}
