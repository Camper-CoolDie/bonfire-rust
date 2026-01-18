use serde_json::json;

use crate::models::Gender;
use crate::raw::RawGender;
use crate::requests::EmptyResponse;
use crate::{Client, Request, Result};

pub(crate) struct SetGenderRequest {
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
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetSex",
                json!({ "sex": self.gender }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }
}
