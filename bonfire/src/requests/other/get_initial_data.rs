use serde::Serialize;

use crate::client::{InfallibleRequest, Request};
use crate::models::InitialData;
use crate::requests::raw::RawInitialData;
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct GetInitialDataRequest {}

impl Request<InitialData> for GetInitialDataRequest {
    type Response = RawInitialData;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<InitialData> {
        client
            .send_request("RAccountsLogin", self, Vec::new())
            .await?
            .try_into()
    }
}
