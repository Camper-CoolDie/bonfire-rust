mod error;
mod paginated;

pub(crate) use error::{InfallibleRequest, RequestError, RequestErrorSource};
pub(crate) use paginated::PaginatedRequest;
use serde::{Deserialize, Serialize};

use crate::{Client, Result};

pub(crate) trait Request<T>: Serialize + Send + Sync {
    type Response: for<'de> Deserialize<'de>;
    type Error: RequestError;

    fn send(&self, client: &Client) -> impl Future<Output = Result<T>> + Send;
}

// Requests that don't return anything should use this struct as the Response type
#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}
