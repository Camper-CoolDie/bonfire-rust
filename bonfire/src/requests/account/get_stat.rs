use serde::Serialize;

use crate::client::{InfallibleRequest, Request};
use crate::requests::raw::account::RawStat;
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct GetStatRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl GetStatRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for GetStatRequest {
    type Response = RawStat;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<RawStat> {
        client
            .send_request("RAccountsGetStory", self, Vec::new())
            .await
    }
}
