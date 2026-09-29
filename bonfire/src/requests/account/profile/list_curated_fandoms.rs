use std::num::NonZeroUsize;

use futures::Stream;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, PaginatedRequest, Request};
use crate::models::Fandom;
use crate::requests::raw::RawFandom;
use crate::{Client, Result, RootError};

const PAGE_SIZE: NonZeroUsize = nonzero!(20usize);

#[derive(Serialize)]
pub(crate) struct ListCuratedFandomsRequest {
    #[serde(rename = "accountId")]
    id: u64,
    offset: usize,
}
impl ListCuratedFandomsRequest {
    pub(crate) fn new(id: u64, offset: usize) -> Self {
        Self { id, offset }
    }

    pub(crate) fn stream(self, client: &Client) -> impl Stream<Item = Result<Fandom>> {
        PaginatedRequest::new(
            client,
            self,
            |request, fandoms| {
                request.offset += fandoms.len();
            },
            PAGE_SIZE,
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    fandoms: Vec<RawFandom>,
}

impl Request<Vec<Fandom>> for ListCuratedFandomsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Fandom>> {
        client
            .send_request("RFandomsGetAllViceroy", self, Vec::new())
            .await?
            .fandoms
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
