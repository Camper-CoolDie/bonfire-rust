use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::{Client, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "contains")]
    is_blocked: bool,
}

impl From<Response> for bool {
    fn from(value: Response) -> Self {
        value.is_blocked
    }
}

#[derive(Serialize)]
pub(crate) struct CheckFandomBlockedRequest {
    #[serde(rename = "fandomId")]
    id: u64,
}
impl CheckFandomBlockedRequest {
    pub(crate) fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Request for CheckFandomBlockedRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RFandomsBlackListContains", self, Vec::new())
            .await
    }
}
