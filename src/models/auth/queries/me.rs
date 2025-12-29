use serde::Deserialize;
use serde_json::json;

use crate::models::{Auth, Me};
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    me: Me,
}

impl Auth {
    pub(crate) async fn _me(client: &mut Client) -> Result<Me> {
        Ok(client
            .send_query::<_, Response>(
                "MeQuery",
                include_str!("graphql/MeQuery.graphql"),
                json!({}),
            )
            .await?
            .me)
    }
}
