use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct UnblockRequest {
    #[serde(rename = "fandomId")]
    id: u64,
}
impl UnblockRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request<()> for UnblockRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<()> {
        client
            .send_request("RFandomsBlackListRemove", self, Vec::new())
            .await?;
        Ok(())
    }
}
