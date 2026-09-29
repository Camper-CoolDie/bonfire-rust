use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct IsAccountBlockedRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl IsAccountBlockedRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "isInBlackList")]
    is_blocked: bool,
}

impl Request<bool> for IsAccountBlockedRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<bool> {
        Ok(client
            .send_request("RAccountsBlackListCheck", self, Vec::new())
            .await?
            .is_blocked)
    }
}
