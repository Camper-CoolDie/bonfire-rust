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
pub(crate) struct SearchAccountsRequest<'a> {
    #[serde(rename = "username", skip_serializing_if = "str::is_empty")]
    query: &'a str,
    offset: usize,
}
impl<'a> SearchAccountsRequest<'a> {
    pub(crate) fn new(query: Option<&'a str>, offset: usize) -> Self {
        Self {
            query: query.unwrap_or(""),
            offset,
        }
    }

    pub(crate) fn stream(self, client: &'a Client) -> impl Stream<Item = Result<Account>> + 'a {
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

impl Request<Vec<Account>> for SearchAccountsRequest<'_> {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Account>> {
        client
            .send_request("RAccountsGetAll", self, Vec::new())
            .await?
            .accounts
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
