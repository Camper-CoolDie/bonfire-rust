use serde::Serialize;

use crate::requests::EmptyResponse;
use crate::{Client, Request, Result};

#[derive(Serialize)]
pub(crate) struct SetAgeRequest {
    age: i64,
}
impl SetAgeRequest {
    pub(crate) fn new(age: Option<i64>) -> Self {
        Self {
            age: age.unwrap_or(0),
        }
    }
}

impl Request for SetAgeRequest {
    type Target = ();

    async fn send_request(&self, client: &Client) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>("RAccountsBioSetAge", self, Vec::default())
            .await?;
        Ok(())
    }
}
