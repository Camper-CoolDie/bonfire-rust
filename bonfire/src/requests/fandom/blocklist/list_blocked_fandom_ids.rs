use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct ListBlockedFandomIdsRequest {
    #[serde(rename = "accountId")]
    id: u64,
}
impl ListBlockedFandomIdsRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "fandomsIds")]
    fandom_ids: Vec<u64>,
}

impl Request<Vec<u64>> for ListBlockedFandomIdsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<u64>> {
        let mut response = client
            .send_request("RAccountsGetIgnoredFandoms", self, Vec::new())
            .await?;

        // The server returns an unsorted list of IDs
        response.fandom_ids.sort_unstable();
        Ok(response.fandom_ids)
    }
}
