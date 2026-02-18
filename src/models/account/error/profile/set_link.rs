use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

/// Represents an error that can occur while setting an external link in an account's profile.
#[derive(Error, Debug)]
pub enum SetLinkError {
    /// The provided title exceeds the maximum allowed length
    #[error("title is too long")]
    TitleTooLong,
    /// The provided URI exceeds the maximum allowed length
    #[error("URI is too long")]
    UriTooLong,
}

impl RequestError for SetLinkError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_TITLE" => {
                Some(SetLinkError::TitleTooLong)
            }
            RootError::Other { code, .. } if code == "E_BAD_URL" => Some(SetLinkError::UriTooLong),
            _ => None,
        })
    }
}
