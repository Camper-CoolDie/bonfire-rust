pub(crate) mod auth;
mod error;
pub(crate) mod profile;
mod raw;

pub use error::{MeliorError, QueryLocation, QueryPath};
use serde::{Deserialize, Serialize};

use crate::client::Request;
pub(crate) use crate::queries::raw::RawMeliorError;

#[derive(Serialize)]
pub(crate) struct MeliorQuery<'a, R: Request> {
    pub variables: &'a R,
    pub query: &'static str,
}

#[derive(Deserialize)]
pub(crate) struct MeliorResponse<R: Request> {
    pub data: Option<R::Response>,
    pub errors: Option<Vec<RawMeliorError>>,
}
