use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::account::PrisonEntry;
use crate::requests::raw::account::RawPrisonEntry;
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "accounts")]
    entries: Vec<RawPrisonEntry>,
}

impl TryFrom<Response> for Vec<PrisonEntry> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.entries.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct GetPrisonRequest {
    offset: u64,
}
impl GetPrisonRequest {
    pub(crate) fn new(offset: u64) -> Self {
        Self { offset }
    }
}

impl Request for GetPrisonRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsPrisonGetAll", self, Vec::new())
            .await
    }
}
