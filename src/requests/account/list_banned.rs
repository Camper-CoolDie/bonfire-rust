use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::account::BanEntry;
use crate::requests::raw::account::RawBanEntry;
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "accounts")]
    entries: Vec<RawBanEntry>,
}

impl TryFrom<Response> for Vec<BanEntry> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.entries.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Serialize)]
pub(crate) struct ListBannedRequest {
    offset: usize,
}
impl ListBannedRequest {
    pub(crate) const PAGE_SIZE: usize = 20;

    pub(crate) fn new(offset: usize) -> Self {
        Self { offset }
    }
}

impl Request for ListBannedRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsPrisonGetAll", self, Vec::new())
            .await
    }
}
