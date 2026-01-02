use serde::Deserialize;
use serde_json::json;

use crate::models::raw::RawAccount;
use crate::models::Account;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    accounts: Vec<RawAccount>,
}

impl Account {
    pub(crate) async fn _search_accounts(
        client: &mut Client,
        name: Option<&str>,
        offset: i64,
        follows_only: bool,
    ) -> Result<Vec<Self>> {
        client
            .send_request::<_, Response>(
                "RAccountsGetAll",
                json!({
                    "username": name,
                    "offset": offset,
                    "isSubscriptionsOnly": follows_only,
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
