use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::publication::kind::sticker::CreateStickerError;
use crate::models::{Publication, Sticker};
use crate::requests::Image;
use crate::requests::image::{Gif, Normal};
use crate::requests::raw::{RawPublication, RawSticker};
use crate::{Client, Result};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateStickerRequest<I: Image> {
    pack_id: u64,
    #[serde(skip)]
    image: I,
}

impl<'a> CreateStickerRequest<Normal<'a>> {
    pub(crate) fn new(pack_id: u64, image: &'a [u8]) -> Self {
        Self {
            pack_id,
            image: Normal::new(image),
        }
    }
}

impl<'a> CreateStickerRequest<Gif<'a>> {
    pub(crate) fn new_gif(pack_id: u64, first_frame: &'a [u8], animated: &'a [u8]) -> Self {
        Self {
            pack_id,
            image: Gif::new(first_frame, animated),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    sticker: RawPublication<RawSticker>,
}

impl<I: Image> Request<Publication<Sticker>> for CreateStickerRequest<I> {
    type Response = Response;
    type Error = CreateStickerError;

    async fn send(&self, client: &Client) -> Result<Publication<Sticker>> {
        let attachments = self.image.attachments();
        client
            .send_request("RStickersAdd", self, attachments)
            .await?
            .sticker
            .try_into()
    }
}
