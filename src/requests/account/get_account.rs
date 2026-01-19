use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::requests::raw::RawAccount;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    pub(crate) account: RawAccount,
}

#[derive(Serialize)]
pub(crate) enum GetAccountRequest<'a> {
    #[serde(rename = "accountId")]
    Id(u64),
    #[serde(rename = "accountName")]
    Name(&'a str),
}
impl<'a> GetAccountRequest<'a> {
    pub(crate) fn new_by_id(id: u64) -> Self {
        Self::Id(id)
    }

    pub(crate) fn new_by_name(name: &'a str) -> Self {
        Self::Name(name)
    }
}

impl Request for GetAccountRequest<'_> {
    type Target = Response;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsGet", self, Vec::default())
            .await
    }
}
