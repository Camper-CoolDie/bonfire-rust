use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Fandom;
use crate::requests::raw::RawFandom;
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    fandoms: Vec<RawFandom>,
}

impl TryFrom<Response> for Vec<Fandom> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.fandoms.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Serialize)]
pub(crate) struct GetSubscriptionsRequest {
    #[serde(rename = "accountId")]
    id: u64,
    offset: u64,
}
impl GetSubscriptionsRequest {
    pub(crate) fn new(id: u64, offset: u64) -> Self {
        Self { id, offset }
    }
}

impl Request for GetSubscriptionsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RFandomsGetAllSubscribed", self, Vec::new())
            .await
    }
}
