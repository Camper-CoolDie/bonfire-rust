use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct UnblockFandomRequest {
    #[serde(rename = "fandomId")]
    id: u64,
}
impl UnblockFandomRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for UnblockFandomRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RFandomsBlackListRemove", self, Vec::new())
            .await
    }
}
