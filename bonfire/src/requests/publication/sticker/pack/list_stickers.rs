use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::{Publication, Sticker};
use crate::requests::raw::{RawPublication, RawSticker};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListStickersRequest {
    pack_id: u64,
}
impl ListStickersRequest {
    pub(crate) fn new(pack_id: u64) -> Self {
        Self { pack_id }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    stickers: Vec<RawPublication<RawSticker>>,
}

impl Request<Vec<Publication<Sticker>>> for ListStickersRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Publication<Sticker>>> {
        client
            .send_request("RStickersGetAllByPackId", self, Vec::new())
            .await?
            .stickers
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
