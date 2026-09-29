use serde::Serialize;

use crate::client::{EmptyResponse, Request};
use crate::models::profile::SetProfileFieldError;
use crate::{Client, Result};

#[derive(Serialize)]
pub(crate) struct SetDescriptionRequest<'a> {
    description: &'a str,
}
impl<'a> SetDescriptionRequest<'a> {
    pub(crate) fn new(description: Option<&'a str>) -> Self {
        Self {
            description: description.unwrap_or(""),
        }
    }
}

impl Request<()> for SetDescriptionRequest<'_> {
    type Response = EmptyResponse;
    type Error = SetProfileFieldError;

    async fn send(&self, client: &Client) -> Result<()> {
        client
            .send_request("RAccountsBioSetDescription", self, Vec::new())
            .await?;
        Ok(())
    }
}
