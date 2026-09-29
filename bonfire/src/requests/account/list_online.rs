use std::num::NonZeroUsize;

use chrono::{DateTime, Utc};
use futures::Stream;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, PaginatedRequest, Request};
use crate::models::Account;
use crate::requests::raw::RawAccount;
use crate::{Client, Result, RootError};

const PAGE_SIZE: NonZeroUsize = nonzero!(50usize);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListOnlineRequest {
    offset_date: i64,
    #[serde(skip)]
    limit_date: i64,
}
impl ListOnlineRequest {
    pub(crate) fn new(offset_date: Option<DateTime<Utc>>) -> Self {
        Self {
            offset_date: offset_date.map_or(0, |date| date.timestamp_millis()),
            limit_date: Utc::now().timestamp_millis(),
        }
    }

    pub(crate) fn stream(self, client: &Client) -> impl Stream<Item = Result<Account>> {
        PaginatedRequest::new(
            client,
            self,
            |request, accounts| {
                if let Some(account) = accounts.last() {
                    request.offset_date = account.last_online_at.timestamp_millis();
                }
            },
            PAGE_SIZE,
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    accounts: Vec<RawAccount>,
}

impl Request<Vec<Account>> for ListOnlineRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Account>> {
        // The server returns a list of accounts sorted by last_online_at in descending order: this
        // would be a big problem when we'll try to load more than one page (because last_online_at
        // is taken only from the last element), so the list is reversed. We also check that it
        // doesn't exceed a statically-defined limit_date to prevent accounts from appearing more
        // than once, even though it's better to get all the pages instantly
        client
            .send_request("RAccountsGetAllOnline", self, Vec::new())
            .await?
            .accounts
            .into_iter()
            .rev()
            .filter_map(|account| {
                (account.last_online_at < self.limit_date).then(|| Account::try_from(account))
            })
            .collect()
    }
}
