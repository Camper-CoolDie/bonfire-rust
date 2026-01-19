use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::requests::raw::RawAccount;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    pub(crate) accounts: Vec<RawAccount>,
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
    type Target = Response;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsGetAll", self, Vec::default())
            .await
    }
}
