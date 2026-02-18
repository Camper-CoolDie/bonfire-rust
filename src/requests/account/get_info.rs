use std::convert::Infallible;

use serde::Serialize;

use crate::client::Request;
use crate::requests::raw::account::RawInfo;
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) enum GetInfoRequest<'a> {
    #[serde(rename = "accountId")]
    Id(u64),
    #[serde(rename = "accountName")]
    Name(&'a str),
}
impl<'a> GetInfoRequest<'a> {
    pub(crate) fn new_by_id(id: u64) -> Self {
        Self::Id(id)
    }

    pub(crate) fn new_by_name(name: &'a str) -> Self {
        Self::Name(name)
    }
}

impl Request for GetInfoRequest<'_> {
    type Response = RawInfo;
    type Error = Infallible;

    async fn send_request(&self, client: &Client) -> Result<RawInfo> {
        client
            .send_request("RAccountsGetProfile", self, Vec::new())
            .await
    }
}
