use chrono::{DateTime, Utc};
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
    pub(crate) async fn _get_online(
        client: &Client,
        offset_date: DateTime<Utc>,
    ) -> Result<Vec<Self>> {
        client
            .send_request::<_, Response>(
                "RAccountsGetAllOnline",
                json!({ "offsetDate": offset_date.timestamp_millis() }),
                Vec::default(),
            )
            .await?
            .accounts
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
