use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Account;
use crate::requests::raw::RawAccount;
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    accounts: Vec<RawAccount>,
}

impl TryFrom<Response> for Vec<Account> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.accounts.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Serialize)]
pub(crate) struct GetFollowsRequest {
    #[serde(rename = "followsOfaAccountId")]
    id: u64,
    offset: u64,
    followers: bool,
}
impl GetFollowsRequest {
    pub(crate) fn new_follows(id: u64, offset: u64) -> Self {
        Self {
            id,
            offset,
            followers: false,
        }
    }

    pub(crate) fn new_followers(id: u64, offset: u64) -> Self {
        Self {
            id,
            offset,
            followers: true,
        }
    }
}

impl Request for GetFollowsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RAccountsFollowsGetAll", self, Vec::new())
            .await
    }
}
