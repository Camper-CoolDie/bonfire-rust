use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct BlockRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl BlockRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for BlockRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsBlackListAdd", self, Vec::new())
            .await
    }
}
