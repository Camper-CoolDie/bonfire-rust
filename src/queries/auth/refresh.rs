use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::auth::RefreshError;
use crate::models::Auth;
use crate::queries::raw::RawAuth;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "loginRefresh")]
    auth: RawAuth,
}

impl From<Response> for Auth {
    fn from(value: Response) -> Self {
        value.auth.into()
    }
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
    type Response = Response;
    type Error = RefreshError;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_query_authless(
                "LoginRefreshMutation",
                include_str!("graphql/LoginRefreshMutation.graphql"),
                self,
            )
            .await
    }
}
