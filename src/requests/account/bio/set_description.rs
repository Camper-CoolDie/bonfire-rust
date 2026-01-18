use serde_json::json;

use crate::requests::EmptyResponse;
use crate::{Client, Request, Result};

pub(crate) struct SetDescriptionRequest<'a> {
    description: &'a str,
}
impl<'a> SetDescriptionRequest<'a> {
    pub(crate) fn new(description: &'a str) -> Self {
        Self { description }
    }
}

impl Request for SetDescriptionRequest<'_> {
    type Target = ();

    async fn send_request(&self, client: &Client) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetDescription",
                json!({ "description": self.description }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }
}
