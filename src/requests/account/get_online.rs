use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Account;
use crate::requests::raw::RawAccount;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    accounts: Vec<RawAccount>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetOnlineRequest {
    offset_date: i64,
}
impl GetOnlineRequest {
    pub(crate) fn new(offset_date: Option<DateTime<Utc>>) -> Self {
        Self {
            offset_date: offset_date
                .map(|datetime| datetime.timestamp_millis())
                .unwrap_or(0),
        }
    }
}

impl Request for GetOnlineRequest {
    type Target = Vec<Account>;

    async fn send_request(&self, client: &Client) -> Result<Vec<Account>> {
        client
            .send_request::<_, Response>("RAccountsGetAllOnline", self, Vec::default())
            .await?
            .accounts
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
