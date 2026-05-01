use serde::Serialize;

use crate::client::{InfallibleRequest, Request};
use crate::requests::raw::RawInitialData;
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct BootstrapRequest<'a> {
    #[serde(rename = "tokenNotification", skip_serializing_if = "str::is_empty")]
    fcm_token: &'a str,
}
impl<'a> BootstrapRequest<'a> {
    pub(crate) fn new(fcm_token: Option<&'a str>) -> Self {
        Self {
            fcm_token: fcm_token.unwrap_or(""),
        }
    }
}

impl Request for BootstrapRequest<'_> {
    type Response = RawInitialData;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<RawInitialData> {
        client
            .send_request("RAccountsLogin", self, Vec::new())
            .await
    }
}
