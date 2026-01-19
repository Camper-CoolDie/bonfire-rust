use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::queries::raw::RawMe;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    pub(crate) me: RawMe,
}

#[derive(Serialize)]
pub(crate) struct MeQuery {}
impl MeQuery {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Request for MeQuery {
    type Target = Response;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_query("MeQuery", include_str!("graphql/MeQuery.graphql"), self)
            .await
    }
}
