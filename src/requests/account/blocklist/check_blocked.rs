use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "isInBlackList")]
    is_blocked: bool,
}

impl From<Response> for bool {
    fn from(value: Response) -> Self {
        value.is_blocked
    }
}

#[derive(Serialize)]
pub(crate) struct CheckBlockedRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl CheckBlockedRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for CheckBlockedRequest {
    type Response = Response;
    type Error = Infallible;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsBlackListCheck", self, Vec::new())
            .await
    }
}
