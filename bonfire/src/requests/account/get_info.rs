use serde::Serialize;

use crate::client::{InfallibleRequest, Request};
use crate::models::AccountInfo;
use crate::requests::raw::RawAccountInfo;
use crate::{Client, Result, RootError};

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

impl Request<AccountInfo> for GetInfoRequest<'_> {
    type Response = RawAccountInfo;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<AccountInfo> {
        client
            .send_request("RAccountsGetProfile", self, Vec::new())
            .await?
            .try_into()
    }
}
