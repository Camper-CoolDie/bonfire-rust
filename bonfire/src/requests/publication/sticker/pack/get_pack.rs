use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::{Publication, StickerPack};
use crate::requests::publication::sticker::pack::ListStickersRequest;
use crate::requests::raw::{RawPublication, RawStickerPack};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum GetPackRequest {
    PackId(u64),
    StickerId(u64),
}
impl GetPackRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self::PackId(id)
    }

    pub(crate) fn new_by_sticker_id(id: u64) -> Self {
        Self::StickerId(id)
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "stickersPack")]
    pack: RawPublication<RawStickerPack>,
}

impl Request<Publication<StickerPack>> for GetPackRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Publication<StickerPack>> {
        let mut pack = Publication::<StickerPack>::try_from(
            client
                .send_request("RStickersPacksGetInfo", self, Vec::new())
                .await?
                .pack,
        )?;

        let stickers = ListStickersRequest::new(pack.id).send(client).await?;
        pack.spec.stickers = stickers;
        Ok(pack)
    }
}
