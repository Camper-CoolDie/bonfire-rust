use serde_json::json;

use crate::models::{Account, EmptyResponse};
use crate::{Client, Result};

impl Account {
    pub(crate) async fn _set_status(client: &Client, status: &str) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsStatusSet",
                json!({ "status": status }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }
}
