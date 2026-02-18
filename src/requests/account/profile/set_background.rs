use std::result::Result as StdResult;

use serde::de::Error as _;
use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::account::SetProfileImageError;
use crate::models::ImageRef;
use crate::requests::raw::RawImageRef;
use crate::{Client, Error, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "image")]
    new_background: RawImageRef,
    #[serde(rename = "imageGif")]
    new_background_gif: Option<RawImageRef>,
}

impl From<Response> for ImageRef {
    fn from(value: Response) -> Self {
        value.new_background.into()
    }
}

impl TryFrom<Response> for (ImageRef, ImageRef) {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        Ok((
            value.new_background.into(),
            value
                .new_background_gif
                .ok_or(serde_json::Error::custom("missing field \"imageGif\""))?
                .into(),
        ))
    }
}

pub(crate) enum SetBackgroundRequest<'a> {
    Normal(&'a [u8]),
    Gif {
        first_frame: &'a [u8],
        animated: &'a [u8],
    },
}
impl<'a> SetBackgroundRequest<'a> {
    pub(crate) fn new(background: &'a [u8]) -> Self {
        Self::Normal(background)
    }

    pub(crate) fn new_gif(first_frame: &'a [u8], animated: &'a [u8]) -> Self {
        Self::Gif {
            first_frame,
            animated,
        }
    }
}

impl Request for SetBackgroundRequest<'_> {
    type Response = Response;
    type Error = SetProfileImageError;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request(
                "RAccountsChangeTitleImage",
                self,
                match self {
                    Self::Normal(background) => vec![background, &[]],
                    Self::Gif {
                        first_frame,
                        animated,
                    } => vec![first_frame, animated],
                },
            )
            .await
    }
}

impl Serialize for SetBackgroundRequest<'_> {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serializes into `{}`
        serializer.serialize_unit_struct("SetBackgroundRequest")
    }
}
