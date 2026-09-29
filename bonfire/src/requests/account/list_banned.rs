use std::num::NonZeroUsize;

use futures::Stream;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, PaginatedRequest, Request};
use crate::models::account::BanEntry;
use crate::requests::raw::account::RawBanEntry;
use crate::{Client, Result, RootError};

const PAGE_SIZE: NonZeroUsize = nonzero!(20usize);

#[derive(Serialize)]
pub(crate) struct ListBannedRequest {
    offset: usize,
}
impl ListBannedRequest {
    pub(crate) fn new(offset: usize) -> Self {
        Self { offset }
    }

    pub(crate) fn stream(self, client: &Client) -> impl Stream<Item = Result<BanEntry>> {
        PaginatedRequest::new(
            client,
            self,
            |request, entries| {
                request.offset += entries.len();
            },
            PAGE_SIZE,
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "accounts")]
    entries: Vec<RawBanEntry>,
}

impl Request<Vec<BanEntry>> for ListBannedRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<BanEntry>> {
        client
            .send_request("RAccountsPrisonGetAll", self, Vec::new())
            .await?
            .entries
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
