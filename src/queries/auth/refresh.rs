use serde::Deserialize;
use serde_json::json;

use crate::models::Auth;
use crate::raw::RawAuth;
use crate::{Client, Query, Result};

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "loginRefresh")]
    auth: RawAuth,
}

pub(crate) struct RefreshQuery<'a> {
    refresh_token: &'a str,
}
impl<'a> RefreshQuery<'a> {
    pub(crate) fn new(refresh_token: &'a str) -> Self {
        Self { refresh_token }
    }
}

impl Query for RefreshQuery<'_> {
    type Target = Auth;

    async fn send_query(&self, client: &Client) -> Result<Auth> {
        Ok(client
            .send_refresh_query::<_, Response>(
                "LoginRefreshMutation",
                include_str!("graphql/LoginRefreshMutation.graphql"),
                json!({ "refreshToken": self.refresh_token }),
            )
            .await?
            .auth
            .into())
    }
}
