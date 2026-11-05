use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

#[derive(Error, Debug)]
pub enum CreateCommentError {
    #[error("invalid data")]
    InvalidData,
    #[error("text is too long")]
    TextTooLong,
    #[error("invalid image")]
    InvalidImage,
    #[error("invalid gif")]
    InvalidGif,
}

impl RequestError for CreateCommentError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_DATA" => {
                Some(CreateCommentError::InvalidData)
            }
            RootError::Other { code, .. } if code == "E_BAD_TEXT_SIZE" => {
                Some(CreateCommentError::TextTooLong)
            }
            RootError::Other { code, .. } if code == "E_BAD_IMAGE" => {
                Some(CreateCommentError::InvalidImage)
            }
            RootError::Other { code, .. } if code == "E_BAD_GIF" => {
                Some(CreateCommentError::InvalidGif)
            }
            _ => None,
        })
    }
}
