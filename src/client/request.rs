use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}

pub(crate) trait Request: Serialize {
    type Response: DeserializeOwned;
    type Target;

    async fn send_request(&self, client: &Client) -> Result<Self::Target>;
}
