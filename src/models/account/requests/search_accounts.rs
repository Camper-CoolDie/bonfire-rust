use serde::Deserialize;
use serde_json::json;

use crate::models::Account;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    accounts: Vec<Account>,
}

impl Account {
    pub(crate) async fn _search_accounts(
        client: &mut Client,
        name: Option<&str>,
        offset: i64,
        follows_only: bool,
    ) -> Result<Vec<Self>> {
        Ok(client
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
            .accounts)
    }
}
