use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Account;
use crate::requests::raw::RawAccount;
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    accounts: Vec<RawAccount>,
    #[serde(skip)]
    limit_date: i64,
}

impl TryFrom<Response> for Vec<Account> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        // The server returns a list of accounts sorted by last_online_at in descending order: this
        // would be a big problem when we'll try to load more than one page (because last_online_at
        // is taken only from the last element), so the list is reversed. We also check that it
        // doesn't exceed a statically-defined limit_date to prevent accounts from appearing more
        // than once, even though it's better to get all the pages instantly
        value
            .accounts
            .into_iter()
            .rev()
            .filter_map(|account| {
                (account.last_online_at <= value.limit_date).then(|| Account::try_from(account))
            })
            .collect()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetOnlineRequest {
    offset_date: i64,
    limit_date: i64,
}
impl GetOnlineRequest {
    pub(crate) const PAGE_SIZE: usize = 50;

    pub(crate) fn new(offset_date: Option<DateTime<Utc>>, limit_date: DateTime<Utc>) -> Self {
        Self {
            offset_date: offset_date.map_or(0, |date| date.timestamp_millis()),
            limit_date: limit_date.timestamp_millis(),
        }
    }
}

impl Request for GetOnlineRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        let mut response = client
            .send_request("RAccountsGetAllOnline", self, Vec::new())
            .await?;

        response.limit_date = self.limit_date;
        Ok(response)
    }
}
