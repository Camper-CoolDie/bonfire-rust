use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::publication::kind::sticker::CreatePackError;
use crate::models::{Publication, StickerPack};
use crate::requests::raw::{RawPublication, RawStickerPack};
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct CreatePackRequest<'a> {
    #[serde(rename = "packName")]
    name: &'a str,
    #[serde(skip)]
    icon: &'a [u8],
}
impl<'a> CreatePackRequest<'a> {
    pub(crate) fn new(name: &'a str, icon: &'a [u8]) -> Self {
        Self { name, icon }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "stickersPack")]
    pack: RawPublication<RawStickerPack>,
}

impl Request<Publication<StickerPack>> for CreatePackRequest<'_> {
    type Response = Response;
    type Error = CreatePackError;

    async fn send(&self, client: &Client) -> Result<Publication<StickerPack>> {
        client
            .send_request("RStickersPackCreate", self, vec![self.icon])
            .await?
            .pack
            .try_into()
    }
}
