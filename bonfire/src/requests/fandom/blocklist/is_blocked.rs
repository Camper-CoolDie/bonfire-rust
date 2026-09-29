use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct IsFandomBlockedRequest {
    #[serde(rename = "fandomId")]
    id: u64,
}
impl IsFandomBlockedRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "contains")]
    is_blocked: bool,
}

impl Request<bool> for IsFandomBlockedRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<bool> {
        Ok(client
            .send_request("RFandomsBlackListContains", self, Vec::new())
            .await?
            .is_blocked)
    }
}
