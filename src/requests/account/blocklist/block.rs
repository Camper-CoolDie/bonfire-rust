use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::{Client, Result};

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

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsBlackListAdd", self, Vec::new())
            .await
    }
}
