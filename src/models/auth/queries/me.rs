use serde::Deserialize;
use serde_json::json;

use crate::models::raw::RawMe;
use crate::models::{Auth, Me};
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    me: RawMe,
}

impl Auth {
    pub(crate) async fn _me(client: &Client) -> Result<Me> {
        Ok(client
            .send_query::<_, Response>(
                "MeQuery",
                include_str!("graphql/MeQuery.graphql"),
                json!({}),
            )
            .await?
            .me
            .into())
    }
}
