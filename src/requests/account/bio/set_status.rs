use serde_json::json;

use crate::requests::EmptyResponse;
use crate::{Client, Request, Result};

pub(crate) struct SetStatusRequest<'a> {
    status: &'a str,
}
impl<'a> SetStatusRequest<'a> {
    pub(crate) fn new(status: &'a str) -> Self {
        Self { status }
    }
}

impl Request for SetStatusRequest<'_> {
    type Target = ();

    async fn send_request(&self, client: &Client) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsStatusSet",
                json!({ "status": self.status }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }
}
