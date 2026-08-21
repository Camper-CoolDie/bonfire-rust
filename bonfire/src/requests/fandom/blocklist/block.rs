use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct BlockFandomRequest {
    #[serde(rename = "fandomId")]
    id: u64,
}
impl BlockFandomRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for BlockFandomRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RFandomsBlackListAdd", self, Vec::new())
            .await
    }
}
