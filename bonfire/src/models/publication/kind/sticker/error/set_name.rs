use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

#[derive(Error, Debug)]
pub enum SetNameError {
    #[error("icon size exceeded")]
    IconSizeExceeded,
    #[error("wrong icon dimensions")]
    WrongIconDimensions,
    #[error("name is too long")]
    TooLong,
}

impl RequestError for SetNameError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_IMAGE_WEIGHT" => {
                Some(SetNameError::IconSizeExceeded)
            }
            RootError::Other { code, .. } if code == "E_BAD_IMAGE_SIZE" => {
                Some(SetNameError::WrongIconDimensions)
            }
            RootError::Other { code, .. } if code == "E_BAD_NAME_SIZE" => {
                Some(SetNameError::TooLong)
            }
            _ => None,
        })
    }
}
