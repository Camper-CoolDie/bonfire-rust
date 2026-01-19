use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::requests::raw::RawAccount;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    pub(crate) accounts: Vec<RawAccount>,
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
    type Response = Response;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsGetAllOnline", self, Vec::default())
            .await
    }
}
