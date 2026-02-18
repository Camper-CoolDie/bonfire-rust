use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "fandomsIds")]
    fandom_ids: Vec<u64>,
}

impl From<Response> for Vec<u64> {
    fn from(mut value: Response) -> Self {
        // The server returns an unsorted list of IDs
        value.fandom_ids.sort_unstable();
        value.fandom_ids
    }
}

#[derive(Serialize)]
pub(crate) struct GetBlockedFandomIdsRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl GetBlockedFandomIdsRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for GetBlockedFandomIdsRequest {
    type Response = Response;
    type Error = Infallible;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsGetIgnoredFandoms", self, Vec::new())
            .await
    }
}
