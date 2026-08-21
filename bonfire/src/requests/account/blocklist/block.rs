use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct BlockAccountRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl BlockAccountRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for BlockAccountRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsBlackListAdd", self, Vec::new())
            .await
    }
}
