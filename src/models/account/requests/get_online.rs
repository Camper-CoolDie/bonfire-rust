use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;

use crate::models::Account;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    accounts: Vec<Account>,
}

impl Account {
    pub(crate) async fn _get_online(
        client: &mut Client,
        offset_date: DateTime<Utc>,
    ) -> Result<Vec<Self>> {
        Ok(client
            .send_request::<_, Response>(
                "RAccountsGetAllOnline",
                json!({ "offsetDate": offset_date.timestamp_millis() }),
                Vec::default(),
            )
            .await?
            .accounts)
    }
}
