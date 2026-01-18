use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Auth;
use crate::queries::raw::RawAuth;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "loginRefresh")]
    auth: RawAuth,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RefreshQuery<'a> {
    refresh_token: &'a str,
}
impl<'a> RefreshQuery<'a> {
    pub(crate) fn new(refresh_token: &'a str) -> Self {
        Self { refresh_token }
    }
}

impl Request for RefreshQuery<'_> {
    type Target = Auth;

    async fn send_request(&self, client: &Client) -> Result<Auth> {
        Ok(client
            .send_refresh_query::<_, Response>(
                "LoginRefreshMutation",
                include_str!("graphql/LoginRefreshMutation.graphql"),
                self,
            )
            .await?
            .auth
            .into())
    }
}
