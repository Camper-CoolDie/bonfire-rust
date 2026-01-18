use serde::{Deserialize, Serialize};

use crate::models::Me;
use crate::queries::raw::RawMe;
use crate::{Client, Query, Result};

#[derive(Deserialize)]
struct Response {
    me: RawMe,
}

#[derive(Serialize)]
pub(crate) struct MeQuery {}
impl MeQuery {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Query for MeQuery {
    type Target = Me;

    async fn send_query(&self, client: &Client) -> Result<Me> {
        Ok(client
            .send_query::<_, Response>("MeQuery", include_str!("graphql/MeQuery.graphql"), self)
            .await?
            .me
            .into())
    }
}
