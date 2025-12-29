use serde_json::json;

use crate::models::{Account, EmptyResponse};
use crate::{Client, Result};

impl Account {
    pub(crate) async fn _set_description(client: &mut Client, description: &str) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetDescription",
                json!({ "description": description }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }
}
