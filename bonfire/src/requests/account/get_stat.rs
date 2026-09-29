use serde::Serialize;

use crate::client::{InfallibleRequest, Request};
use crate::models::AccountStat;
use crate::requests::raw::RawAccountStat;
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

impl Request<AccountStat> for GetStatRequest {
    type Response = RawAccountStat;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<AccountStat> {
        Ok(client
            .send_request("RAccountsGetStory", self, Vec::new())
            .await?
            .into())
    }
}
