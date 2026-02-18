use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::models::Gender;
use crate::requests::raw::RawGender;
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct SetGenderRequest {
    #[serde(rename = "sex")]
    gender: RawGender,
}
impl SetGenderRequest {
    pub(crate) fn new(gender: Gender) -> Self {
        Self {
            gender: RawGender::from(gender),
        }
    }
}

impl Request for SetGenderRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client
            .send_request("RAccountsBioSetSex", self, Vec::new())
            .await
    }
}
