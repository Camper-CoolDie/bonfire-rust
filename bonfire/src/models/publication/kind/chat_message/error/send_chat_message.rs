use thiserror::Error;

use crate::client::RequestError;
use crate::{Result, RootError};

#[derive(Error, Debug)]
pub enum SendChatMessageError {
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

impl RequestError for SendChatMessageError {
    type Source = RootError;

    fn try_convert(error: &RootError) -> Result<Option<Self>> {
        Ok(match error {
            RootError::Other { code, .. } if code == "E_BAD_DATA" => {
                Some(SendChatMessageError::InvalidData)
            }
            RootError::Other { code, .. } if code == "E_BAD_TEXT" => {
                Some(SendChatMessageError::TextTooLong)
            }
            RootError::Other { code, .. } if code == "E_BAD_IMAGE" => {
                Some(SendChatMessageError::InvalidImage)
            }
            RootError::Other { code, .. } if code == "E_BAD_GIF" => {
                Some(SendChatMessageError::InvalidGif)
            }
            RootError::Other { code, .. } if code == "E_BLACK_LIST" => {
                Some(SendChatMessageError::Blocked)
            }
            RootError::Other { code, .. } if code == "E_IS_IGNORE_VOICE_MESSAGES" => {
                Some(SendChatMessageError::VoiceBlocked)
            }
            _ => None,
        })
    }
}
