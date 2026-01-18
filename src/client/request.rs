use serde::Deserialize;

use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}

pub(crate) trait Request {
    type Target;

    async fn send_request(&self, client: &Client) -> Result<Self::Target>;
}
