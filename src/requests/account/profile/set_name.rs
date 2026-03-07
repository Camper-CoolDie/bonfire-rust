use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::models::profile::SetNameError;
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct SetNameRequest<'a> {
    name: &'a str,
}
impl<'a> SetNameRequest<'a> {
    pub(crate) fn new(name: &'a str) -> Self {
        Self { name }
    }
}

impl Request for SetNameRequest<'_> {
    type Response = EmptyResponse;
    type Error = SetNameError;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsChangeName", self, Vec::new())
            .await
    }
}
