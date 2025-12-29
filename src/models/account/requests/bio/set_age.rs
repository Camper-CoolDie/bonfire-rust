use serde_json::json;

use crate::models::{Account, EmptyResponse};
use crate::{Client, Result};

impl Account {
    pub(crate) async fn _set_age(client: &mut Client, age: i64) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetAge",
                json!({ "age": age }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }
}
