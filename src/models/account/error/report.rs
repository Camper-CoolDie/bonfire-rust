use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

/// Represents errors that can occur while reporting an account.
#[derive(Error, Debug)]
pub enum ReportError {
    /// The account has already been reported.
    #[error("already reported")]
    AlreadyReported,
}

impl RequestError for ReportError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_EXIST" => {
                Some(ReportError::AlreadyReported)
            }
            _ => None,
        })
    }
}
