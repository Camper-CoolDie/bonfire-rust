use std::result::Result as StdResult;

use serde::{Serialize, Serializer};

use crate::client::Request;
use crate::models::profile::SetProfileImageError;
use crate::requests::raw::Image;
use crate::requests::raw::image::{Gif, Normal};
use crate::{Client, Result};

pub(crate) struct SetBackgroundRequest<I: Image> {
    background: I,
}

impl<'a> SetBackgroundRequest<Normal<'a>> {
    pub(crate) fn new(background: &'a [u8]) -> Self {
        SetBackgroundRequest {
            background: Normal::new(background),
        }
    }
}

impl<'a> SetBackgroundRequest<Gif<'a>> {
    pub(crate) fn new_gif(first_frame: &'a [u8], animated: &'a [u8]) -> Self {
        SetBackgroundRequest {
            background: Gif::new(first_frame, animated),
        }
    }
}

impl<I: Image> Serialize for SetBackgroundRequest<I> {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serializes into `{}`
        serializer.serialize_unit_struct("SetBackgroundRequest")
    }
}

impl<I: Image> Request<I::Target> for SetBackgroundRequest<I> {
    type Response = I::Response;
    type Error = SetProfileImageError;

    async fn send(&self, client: &Client) -> Result<I::Target> {
        let attachments = self.background.attachments();
        let response = client
            .send_request("RAccountsChangeTitleImage", self, attachments)
            .await?;
        self.background.parse_response(response)
    }
}
