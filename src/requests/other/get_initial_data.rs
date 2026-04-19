use serde::Serialize;

use crate::client::{InfallibleRequest, Request};
use crate::requests::raw::RawInitialData;
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct GetInitialDataRequest {}
impl GetInitialDataRequest {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Request for GetInitialDataRequest {
    type Response = RawInitialData;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<RawInitialData> {
        client
            .send_request("RAccountsLogin", self, Vec::new())
            .await
    }
}
