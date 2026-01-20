use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct SetAgeRequest {
    age: i64,
}
impl SetAgeRequest {
    pub(crate) fn new(age: Option<i64>) -> Self {
        Self {
            age: age.unwrap_or(0),
        }
    }
}

impl Request for SetAgeRequest {
    type Response = EmptyResponse;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsBioSetAge", self, Vec::new())
            .await
    }
}
