use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

#[derive(Error, Debug)]
pub enum CreateStickerError {
    #[error("size exceeded")]
    SizeExceeded,
    #[error("dimensions are too high")]
    DimensionsTooHigh,
    #[error("too many stickers")]
    TooManyStickers,
}

impl RequestError for CreateStickerError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_IMG_WEIGHT" => {
                Some(CreateStickerError::SizeExceeded)
            }
            RootError::Other { code, .. } if code == "E_BAD_IMG_SIDES" => {
                Some(CreateStickerError::DimensionsTooHigh)
            }
            RootError::Other { code, .. } if code == "E_BAD_STICKERS_COUNT" => {
                Some(CreateStickerError::TooManyStickers)
            }
            _ => None,
        })
    }
}
