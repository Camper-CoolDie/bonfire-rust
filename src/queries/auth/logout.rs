use serde::Serialize;

use crate::queries::EmptyResponse;
use crate::{Client, Query, Result};

#[derive(Serialize)]
pub(crate) struct LogoutQuery {}
impl LogoutQuery {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Query for LogoutQuery {
    type Target = ();

    async fn send_query(&self, client: &Client) -> Result<()> {
        client
            .send_query::<_, EmptyResponse>(
                "LogoutMutation",
                include_str!("graphql/LogoutMutation.graphql"),
                self,
            )
            .await?;
        Ok(())
    }
}
