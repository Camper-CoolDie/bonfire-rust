use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

#[derive(Error, Debug)]
pub enum CreateChatMessageError {
    #[error("invalid data")]
    InvalidData,
    #[error("text is too long")]
    TextTooLong,
    #[error("invalid image")]
    InvalidImage,
    #[error("invalid gif")]
    InvalidGif,
    #[error("message is blocked")]
    Blocked,
    #[error("voice message is blocked")]
    VoiceBlocked,
}

impl RequestError for CreateChatMessageError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_DATA" => {
                Some(CreateChatMessageError::InvalidData)
            }
            RootError::Other { code, .. } if code == "E_BAD_TEXT" => {
                Some(CreateChatMessageError::TextTooLong)
            }
            RootError::Other { code, .. } if code == "E_BAD_IMAGE" => {
                Some(CreateChatMessageError::InvalidImage)
            }
            RootError::Other { code, .. } if code == "E_BAD_GIF" => {
                Some(CreateChatMessageError::InvalidGif)
            }
            RootError::Other { code, .. } if code == "E_BLACK_LIST" => {
                Some(CreateChatMessageError::Blocked)
            }
            RootError::Other { code, .. } if code == "E_IS_IGNORE_VOICE_MESSAGES" => {
                Some(CreateChatMessageError::VoiceBlocked)
            }
            _ => None,
        })
    }
}
