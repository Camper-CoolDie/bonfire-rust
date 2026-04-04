use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::models::auth::LogoutError;
use crate::queries::GRAPHQL_DIR;
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct LogoutQuery {}
impl LogoutQuery {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Request for LogoutQuery {
    type Response = EmptyResponse;
    type Error = LogoutError;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        let graphql = GRAPHQL_DIR
            .get_file("auth/LogoutMutation.graphql")
            .and_then(|file| file.contents_utf8())
            .expect("failed to retrieve graphql query");

        client.send_query("LogoutMutation", graphql, self).await
    }
}
