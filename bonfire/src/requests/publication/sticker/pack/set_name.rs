use serde::{Deserialize, Serialize};

use crate::client::{EmptyResponse, Request};
use crate::models::publication::kind::sticker::SetNameError;
use crate::models::{ImageRef, Publication, StickerPack};
use crate::requests::raw::{RawPublication, RawStickerPack};
use crate::{Client, Result};

const REQUEST_NAME: &str = "RStickersPackChange";

#[derive(Serialize)]
pub(crate) struct SetNameRequest<'a, I> {
    #[serde(rename = "unitId")]
    id: u64,
    #[serde(rename = "packName")]
    name: &'a str,
    #[serde(skip)]
    icon: I,
}

impl<'a> SetNameRequest<'a, ()> {
    pub(crate) fn new(id: u64, name: &'a str) -> Self {
        Self { id, name, icon: () }
    }
}

impl<'a> SetNameRequest<'a, &'a [u8]> {
    pub(crate) fn new_with_icon(id: u64, name: &'a str, icon: &'a [u8]) -> Self {
        Self { id, name, icon }
    }
}

impl Request<()> for SetNameRequest<'_, ()> {
    type Response = EmptyResponse;
    type Error = SetNameError;

    async fn send(&self, client: &Client) -> Result<()> {
        client.send_request(REQUEST_NAME, self, vec![&[]]).await?;
        Ok(())
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "stickersPack")]
    pack: RawPublication<RawStickerPack>,
}

impl Request<ImageRef> for SetNameRequest<'_, &[u8]> {
    type Response = Response;
    type Error = SetNameError;

    async fn send(&self, client: &Client) -> Result<ImageRef> {
        let pack = Publication::<StickerPack>::try_from(
            client
                .send_request(REQUEST_NAME, self, vec![self.icon])
                .await?
                .pack,
        )?;
        Ok(pack.spec.icon)
    }
}
