use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Account;
use crate::requests::raw::RawAccount;
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    accounts: Vec<RawAccount>,
}

impl TryFrom<Response> for Vec<Account> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.accounts.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetOnlineRequest {
    offset_date: i64,
}
impl GetOnlineRequest {
    pub(crate) fn new(offset_date: Option<DateTime<Utc>>) -> Self {
        Self {
            offset_date: offset_date.map_or(0, |date| date.timestamp_millis()),
        }
    }
}

impl Request for GetOnlineRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsGetAllOnline", self, Vec::new())
            .await
    }
}
