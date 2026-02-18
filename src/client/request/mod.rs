mod error;

pub(crate) use error::{InfallibleRequest, RequestError, RequestErrorSource};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{Client, Result};

pub(crate) trait Request: Serialize {
    type Response: DeserializeOwned;
    type Error: RequestError;

    async fn send_request(&self, client: &Client) -> Result<Self::Response>;
}

// Requests that don't return anything should use this struct as the Response type
#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}
