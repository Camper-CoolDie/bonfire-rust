use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::{Client, MeliorError, Result};

#[derive(Serialize)]
pub(crate) struct SetTokenQuery<'a> {
    token: &'a str,
}
impl<'a> SetTokenQuery<'a> {
    pub(crate) fn new(token: &'a str) -> Self {
        Self { token }
    }
}

impl Request for SetTokenQuery<'_> {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<MeliorError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_query(
                "SetNotificationToken",
                "notification/SetNotificationToken.graphql",
                self,
            )
            .await
    }
}
