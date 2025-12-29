use serde::Deserialize;
use serde_json::json;

use crate::models::Account;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    account: Account,
}

impl Account {
    pub(crate) async fn _get_account(
        client: &mut Client,
        id: Option<i64>,
        name: Option<&str>,
    ) -> Result<Self> {
        Ok(client
            .send_request::<_, Response>(
                "RAccountsGet",
                json!({
                    "accountId": id,
                    "accountName": name
                }),
                Vec::default(),
            )
            .await?
            .account)
    }
}
