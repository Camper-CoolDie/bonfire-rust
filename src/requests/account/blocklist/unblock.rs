use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct UnblockRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl UnblockRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for UnblockRequest {
    type Response = EmptyResponse;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsBlackListRemove", self, Vec::new())
            .await
    }
}
