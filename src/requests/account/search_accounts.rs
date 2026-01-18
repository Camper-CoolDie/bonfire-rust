use serde::Deserialize;
use serde_json::json;

use crate::models::Account;
use crate::raw::RawAccount;
use crate::{Client, Request, Result};

#[derive(Deserialize)]
struct Response {
    accounts: Vec<RawAccount>,
}

pub(crate) struct SearchAccountsRequest<'a> {
    name: Option<&'a str>,
    offset: i64,
    follows_only: bool,
}
impl<'a> SearchAccountsRequest<'a> {
    pub(crate) fn new(name: Option<&'a str>, offset: i64, follows_only: bool) -> Self {
        Self {
            name,
            offset,
            follows_only,
        }
    }
}

impl Request for SearchAccountsRequest<'_> {
    type Target = Vec<Account>;

    async fn send_request(&self, client: &Client) -> Result<Vec<Account>> {
        client
            .send_request::<_, Response>(
                "RAccountsGetAll",
                json!({
                    "username": self.name,
                    "offset": self.offset,
                    "isSubscriptionsOnly": self.follows_only,
                }),
                Vec::default(),
            )
            .await?
            .accounts
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
