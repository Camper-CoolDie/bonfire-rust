use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::{Post, PostTag, Publication};
use crate::requests::raw::{RawPost, RawPostTag, RawPublication};
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "unit")]
    post: RawPublication<RawPost>,
    tags: Vec<RawPublication<RawPostTag>>,
}

impl TryFrom<Response> for (Publication<Post>, Vec<Publication<PostTag>>) {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        Ok((
            value.post.try_into()?,
            value
                .tags
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
        ))
    }
}

#[derive(Serialize)]
pub(crate) struct GetPostRequest {
    #[serde(rename = "unitId")]
    id: u64,
}
impl GetPostRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for GetPostRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client.send_request("RPostGet", self, Vec::new()).await
    }
}
