use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Account;
use crate::requests::raw::RawAccount;
use crate::{Client, Error, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    accounts: Vec<RawAccount>,
}

impl TryFrom<Response> for Vec<Account> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.accounts.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Serialize)]
pub(crate) struct SearchAccountsRequest<'a> {
    #[serde(rename = "username", skip_serializing_if = "str::is_empty")]
    name: &'a str,
    offset: u64,
    #[serde(rename = "isSubscriptionsOnly")]
    follows_only: bool,
}
impl<'a> SearchAccountsRequest<'a> {
    pub(crate) fn new(name: Option<&'a str>, offset: u64, follows_only: bool) -> Self {
        Self {
            name: name.unwrap_or(""),
            offset,
            follows_only,
        }
    }
}

impl Request for SearchAccountsRequest<'_> {
    type Response = Response;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsGetAll", self, Vec::new())
            .await
    }
}
