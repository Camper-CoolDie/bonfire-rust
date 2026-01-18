pub(crate) mod auth;
mod error;
pub(crate) mod profile;
mod raw;

pub use error::{MeliorError, QueryLocation, QueryPath};
use serde::{Deserialize, Serialize};

use crate::queries::raw::RawMeliorError;
use crate::{Client, Result};

#[derive(Deserialize)]
struct EmptyResponse {}

pub(crate) trait Query {
    type Target;

    async fn send_query(&self, client: &Client) -> Result<Self::Target>;
}

#[derive(Serialize)]
pub(crate) struct MeliorQuery<R> {
    pub variables: R,
    pub query: &'static str,
}

#[derive(Deserialize)]
pub(crate) struct MeliorResponse<S> {
    pub data: Option<S>,
    pub errors: Option<Vec<RawMeliorError>>,
}
