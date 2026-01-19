use serde::Serialize;

use crate::client::{EmptyResponse, Request};
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

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_query(
                "LogoutMutation",
                include_str!("graphql/LogoutMutation.graphql"),
                self,
            )
            .await
    }
}
