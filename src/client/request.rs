use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}

pub(crate) trait Request
where
    Self: Serialize,
{
    type Response: DeserializeOwned;

    async fn send_request(&self, client: &Client) -> Result<Self::Response>;
}
