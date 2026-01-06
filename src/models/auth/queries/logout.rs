use serde_json::json;

use crate::models::{Auth, EmptyResponse};
use crate::{Client, Result};

impl Auth {
    pub(crate) async fn _logout(client: &Client) -> Result<()> {
        client
            .send_query::<_, EmptyResponse>(
                "LogoutMutation",
                include_str!("graphql/LogoutMutation.graphql"),
                json!({}),
            )
            .await?;
        Ok(())
    }
}
