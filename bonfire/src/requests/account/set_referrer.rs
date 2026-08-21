use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::models::account::SetReferrerError;
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct SetReferrerRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl SetReferrerRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for SetReferrerRequest {
    type Response = EmptyResponse;
    type Error = SetReferrerError;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsSetRecruiter", self, Vec::new())
            .await
    }
}
