use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::profile::SetProfileImageError;
use crate::models::ImageRef;
use crate::requests::raw::RawImageRef;
use crate::{Client, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Response {
    new_avatar: RawImageRef,
}

impl From<Response> for ImageRef {
    fn from(value: Response) -> Self {
        value.new_avatar.into()
    }
}

#[derive(Serialize)]
pub(crate) struct SetAvatarRequest<'a> {
    #[serde(skip)]
    avatar: &'a [u8],
}
impl<'a> SetAvatarRequest<'a> {
    pub(crate) fn new(avatar: &'a [u8]) -> Self {
        Self { avatar }
    }
}

impl Request for SetAvatarRequest<'_> {
    type Response = Response;
    type Error = SetProfileImageError;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsChangeAvatar", self, vec![self.avatar])
            .await
    }
}
