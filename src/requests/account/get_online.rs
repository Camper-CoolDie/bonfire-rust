use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;

use crate::models::Account;
use crate::raw::RawAccount;
use crate::{Client, Request, Result};

#[derive(Deserialize)]
struct Response {
    accounts: Vec<RawAccount>,
}

pub(crate) struct GetOnlineRequest {
    offset_date: DateTime<Utc>,
}
impl GetOnlineRequest {
    pub(crate) fn new(offset_date: DateTime<Utc>) -> Self {
        Self { offset_date }
    }
}

impl Request for GetOnlineRequest {
    type Target = Vec<Account>;

    async fn send_request(&self, client: &Client) -> Result<Vec<Account>> {
        client
            .send_request::<_, Response>(
                "RAccountsGetAllOnline",
                json!({ "offsetDate": self.offset_date.timestamp_millis() }),
                Vec::default(),
            )
            .await?
            .accounts
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
