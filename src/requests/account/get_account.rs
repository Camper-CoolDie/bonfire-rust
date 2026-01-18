use serde::Deserialize;
use serde_json::json;

use crate::models::Account;
use crate::raw::RawAccount;
use crate::{Client, Request, Result};

#[derive(Deserialize)]
struct Response {
    account: RawAccount,
}

pub(crate) struct GetAccountRequest<'a> {
    id: Option<u64>,
    name: Option<&'a str>,
}
impl<'a> GetAccountRequest<'a> {
    pub(crate) fn new_by_id(id: u64) -> Self {
        Self {
            id: Some(id),
            name: None,
        }
    }

    pub(crate) fn new_by_name(name: &'a str) -> Self {
        Self {
            id: None,
            name: Some(name),
        }
    }
}

impl Request for GetAccountRequest<'_> {
    type Target = Account;

    async fn send_request(&self, client: &Client) -> Result<Account> {
        client
            .send_request::<_, Response>(
                "RAccountsGet",
                json!({
                    "accountId": self.id,
                    "accountName": self.name,
                }),
                Vec::default(),
            )
            .await?
            .account
            .try_into()
    }
}
