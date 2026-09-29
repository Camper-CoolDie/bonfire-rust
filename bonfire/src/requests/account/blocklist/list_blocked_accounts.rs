use std::num::NonZeroUsize;

use futures::Stream;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, PaginatedRequest, Request};
use crate::models::Account;
use crate::requests::raw::RawAccount;
use crate::{Client, Result, RootError};

const PAGE_SIZE: NonZeroUsize = nonzero!(20usize);

#[derive(Serialize)]
pub(crate) struct ListBlockedAccountsRequest {
    #[serde(rename = "accountId")]
    id: u64,
    offset: usize,
}
impl ListBlockedAccountsRequest {
    pub(crate) fn new(id: u64, offset: usize) -> Self {
        Self { id, offset }
    }

    pub(crate) fn stream(self, client: &Client) -> impl Stream<Item = Result<Account>> {
        PaginatedRequest::new(
            client,
            self,
            |request, accounts| {
                request.offset += accounts.len();
            },
            PAGE_SIZE,
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    accounts: Vec<RawAccount>,
}

impl Request<Vec<Account>> for ListBlockedAccountsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Account>> {
        client
            .send_request("RAccountsBlackListGetAll", self, Vec::new())
            .await?
            .accounts
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
