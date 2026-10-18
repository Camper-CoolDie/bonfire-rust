use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

#[derive(Error, Debug)]
pub enum CreatePackError {
    #[error("icon size exceeded")]
    IconSizeExceeded,
    #[error("wrong icon dimensions")]
    WrongIconDimensions,
    #[error("name is too long")]
    NameTooLong,
    #[error("too many packs")]
    TooManyPacks,
}

impl RequestError for CreatePackError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_IMAGE_WEIGHT" => {
                Some(CreatePackError::IconSizeExceeded)
            }
            RootError::Other { code, .. } if code == "E_BAD_IMAGE_SIZE" => {
                Some(CreatePackError::WrongIconDimensions)
            }
            RootError::Other { code, .. } if code == "E_BAD_NAME_SIZE" => {
                Some(CreatePackError::NameTooLong)
            }
            RootError::Other { code, .. } if code == "E_TOO_MANY" => {
                Some(CreatePackError::TooManyPacks)
            }
            _ => None,
        })
    }
}
