use serde::Serialize;

use crate::requests::EmptyResponse;
use crate::{Client, Request, Result};

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
    type Target = ();

    async fn send_request(&self, client: &Client) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>("RAccountsStatusSet", self, Vec::default())
            .await?;
        Ok(())
    }
}
