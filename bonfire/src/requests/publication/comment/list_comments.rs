use std::num::NonZeroUsize;

use chrono::{DateTime, Utc};
use futures::Stream;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, PaginatedRequest, Request};
use crate::models::{Comment, Publication};
use crate::requests::raw::{RawComment, RawPublication};
use crate::{Client, Result, RootError};

const PAGE_SIZE: NonZeroUsize = nonzero!(50usize);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListCommentsRequest {
    #[serde(rename = "unitId")]
    parent_id: u64,
    offset_date: i64,
}
impl ListCommentsRequest {
    pub(crate) fn new(parent_id: u64, offset_date: Option<DateTime<Utc>>) -> Self {
        Self {
            parent_id,
            offset_date: offset_date.map_or(0, |date| date.timestamp_millis()),
        }
    }

    pub(crate) fn stream(
        self,
        client: &Client,
    ) -> impl Stream<Item = Result<Publication<Comment>>> {
        PaginatedRequest::new(
            client,
            self,
            |request, comments| {
                if let Some(comment) = comments.last() {
                    request.offset_date = comment.created_at.timestamp_millis();
                }
            },
            PAGE_SIZE,
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "units")]
    comments: Vec<RawPublication<RawComment>>,
}

impl Request<Vec<Publication<Comment>>> for ListCommentsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Publication<Comment>>> {
        // Received list is always sorted from newest to oldest
        client
            .send_request("RCommentsGetAll", self, Vec::new())
            .await?
            .comments
            .into_iter()
            .map(TryInto::try_into)
            .collect()
    }
}
