use std::convert::Infallible;

use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Me;
use crate::queries::raw::RawMe;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    me: RawMe,
}

impl From<Response> for Me {
    fn from(value: Response) -> Self {
        value.me.into()
    }
}

#[derive(Serialize)]
pub(crate) struct MeQuery {}
impl MeQuery {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Request for MeQuery {
    type Response = Response;
    type Error = Infallible;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_query("MeQuery", include_str!("graphql/MeQuery.graphql"), self)
            .await
    }
}
