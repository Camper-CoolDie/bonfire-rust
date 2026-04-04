use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Auth;
use crate::models::auth::RefreshError;
use crate::queries::GRAPHQL_DIR;
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
        let graphql = GRAPHQL_DIR
            .get_file("auth/LoginRefreshMutation.graphql")
            .and_then(|file| file.contents_utf8())
            .expect("failed to retrieve graphql query");

        client
            .send_query_authless("LoginRefreshMutation", graphql, self)
            .await
    }
}
