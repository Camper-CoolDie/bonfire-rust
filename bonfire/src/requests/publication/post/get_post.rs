use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::{Post, PostTag, Publication};
use crate::requests::raw::{RawPost, RawPostTag, RawPublication};
use crate::{Client, Result, RootError};

type Target = (Publication<Post>, Vec<Publication<PostTag>>);

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

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "unit")]
    post: RawPublication<RawPost>,
    tags: Vec<RawPublication<RawPostTag>>,
}

impl Request<Target> for GetPostRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Target> {
        let response = client.send_request("RPostGet", self, Vec::new()).await?;

        Ok((
            response.post.try_into()?,
            response
                .tags
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
        ))
    }
}
