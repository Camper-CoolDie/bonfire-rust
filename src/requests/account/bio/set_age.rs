use serde_json::json;

use crate::requests::EmptyResponse;
use crate::{Client, Request, Result};

pub(crate) struct SetAgeRequest {
    age: i64,
}
impl SetAgeRequest {
    pub(crate) fn new(age: i64) -> Self {
        Self { age }
    }
}

impl Request for SetAgeRequest {
    type Target = ();

    async fn send_request(&self, client: &Client) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetAge",
                json!({ "age": self.age }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }
}
