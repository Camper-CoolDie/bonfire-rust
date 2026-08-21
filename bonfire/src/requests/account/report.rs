use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::models::account::ReportError;
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct ReportRequest<'a> {
    #[serde(rename = "accountId")]
    id: u64,
    comment: &'a str,
}
impl<'a> ReportRequest<'a> {
    pub(crate) fn new(id: u64, comment: &'a str) -> Self {
        Self { id, comment }
    }
}

impl Request for ReportRequest<'_> {
    type Response = EmptyResponse;
    type Error = ReportError;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsReport", self, Vec::new())
            .await
    }
}
