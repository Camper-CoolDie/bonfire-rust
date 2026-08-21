use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::models::Settings;
use crate::requests::raw::RawSettings;
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct SaveSettingsRequest {
    settings: RawSettings,
}
impl SaveSettingsRequest {
    pub(crate) fn new(settings: Settings) -> Result<Self> {
        Ok(Self {
            settings: settings.try_into()?,
        })
    }
}

impl Request for SaveSettingsRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountSetSettings", self, Vec::new())
            .await
    }
}
