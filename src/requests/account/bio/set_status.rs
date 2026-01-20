use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct SetStatusRequest<'a> {
    status: &'a str,
}
impl<'a> SetStatusRequest<'a> {
    pub(crate) fn new(status: Option<&'a str>) -> Self {
        Self {
            status: status.unwrap_or(""),
        }
    }
}

impl Request for SetStatusRequest<'_> {
    type Response = EmptyResponse;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsStatusSet", self, Vec::new())
            .await
    }
}
