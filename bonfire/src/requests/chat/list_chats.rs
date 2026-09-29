use std::num::NonZeroUsize;

use futures::Stream;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, PaginatedRequest, Request};
use crate::models::Chat;
use crate::requests::raw::RawChat;
use crate::{Client, Result, RootError};

const PAGE_SIZE: NonZeroUsize = nonzero!(10usize);

#[derive(Serialize)]
pub(crate) struct ListChatsRequest {
    offset: usize,
}
impl ListChatsRequest {
    pub(crate) fn new(offset: usize) -> Self {
        Self { offset }
    }

    pub(crate) fn stream(self, client: &Client) -> impl Stream<Item = Result<Chat>> {
        PaginatedRequest::new(
            client,
            self,
            |request, chats| {
                request.offset += chats.len();
            },
            PAGE_SIZE,
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "units")]
    chats: Vec<RawChat>,
}

impl Request<Vec<Chat>> for ListChatsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Chat>> {
        client
            .send_request("RChatsGetAll", self, Vec::new())
            .await?
            .chats
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
