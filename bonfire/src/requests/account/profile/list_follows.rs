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
pub(crate) struct ListFollowsRequest {
    #[serde(rename = "followsOfaAccountId")]
    id: u64,
    offset: usize,
    followers: bool,
}
impl ListFollowsRequest {
    pub(crate) fn new_follows(id: u64, offset: usize) -> Self {
        Self {
            id,
            offset,
            followers: false,
        }
    }

    pub(crate) fn new_followers(id: u64, offset: usize) -> Self {
        Self {
            id,
            offset,
            followers: true,
        }
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

impl Request<Vec<Account>> for ListFollowsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Account>> {
        client
            .send_request("RAccountsFollowsGetAll", self, Vec::new())
            .await?
            .accounts
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
