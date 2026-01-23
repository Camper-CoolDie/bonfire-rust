use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct SetLinkRequest<'a> {
    index: u32,
    title: &'a str,
    #[serde(rename = "url")]
    uri: &'a str,
}
impl<'a> SetLinkRequest<'a> {
    pub(crate) fn new(index: u32, title: &'a str, uri: &'a str) -> Self {
        Self { index, title, uri }
    }
}

impl Request for SetLinkRequest<'_> {
    type Response = EmptyResponse;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsBioSetLink", self, Vec::new())
            .await
    }
}
