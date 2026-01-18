use serde::Serialize;

use crate::models::Gender;
use crate::requests::raw::RawGender;
use crate::requests::EmptyResponse;
use crate::{Client, Request, Result};

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
    type Target = ();

    async fn send_request(&self, client: &Client) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>("RAccountsBioSetSex", self, Vec::default())
            .await?;
        Ok(())
    }
}
