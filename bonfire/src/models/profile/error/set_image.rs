use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

/// Represents an error that can occur while setting an account's avatar or profile background.
#[derive(Error, Debug)]
pub enum SetProfileImageError {
    /// The provided image file size exceeds the server's limit
    #[error("size exceeded")]
    SizeExceeded,
    /// The provided image dimensions (width/height) exceed the server's limit
    #[error("dimensions are too high")]
    DimensionsTooHigh,
}

impl RequestError for SetProfileImageError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_IMG_WEIGHT" => {
                Some(SetProfileImageError::SizeExceeded)
            }
            RootError::Other { code, .. } if code == "E_BAD_IMG_SIDES" => {
                Some(SetProfileImageError::DimensionsTooHigh)
            }
            _ => None,
        })
    }
}
