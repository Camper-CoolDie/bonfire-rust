use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Account;
use crate::requests::raw::RawAccount;
use crate::{Client, Result, RootError};

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

#[derive(Deserialize)]
pub(crate) struct Response {
    account: RawAccount,
}

impl Request<Account> for GetAccountRequest<'_> {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Account> {
        client
            .send_request("RAccountsGet", self, Vec::new())
            .await?
            .account
            .try_into()
    }
}
