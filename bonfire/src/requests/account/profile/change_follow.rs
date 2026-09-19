use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct ChangeFollowRequest {
    #[serde(rename = "accountId")]
    pub id: u64,
    pub follow: bool,
}
impl ChangeFollowRequest {
    pub(crate) fn new_follow(id: u64) -> Self {
        Self { id, follow: true }
    }

    pub(crate) fn new_unfollow(id: u64) -> Self {
        Self { id, follow: false }
    }
}

impl Request for ChangeFollowRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsFollowsChange", self, Vec::new())
            .await
    }
}
