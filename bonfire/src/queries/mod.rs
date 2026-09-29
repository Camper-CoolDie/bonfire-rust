pub(crate) mod auth;
mod error;
pub(crate) mod notification;
pub(crate) mod profile;
pub(crate) mod raw;

use std::marker::PhantomData;

pub use error::{MeliorError, QueryLocation, QueryPath};
use serde::{Deserialize, Serialize};

use crate::client::Request;
pub(crate) use crate::queries::raw::RawMeliorError;

#[derive(Serialize)]
pub(crate) struct MeliorQuery<'a, T, R: Request<T>> {
    pub operation_name: &'static str,
    pub variables: &'a R,
    pub query: &'static str,
    #[serde(skip)]
    pub _marker: PhantomData<T>,
}

#[derive(Deserialize)]
pub(crate) struct MeliorResponse<T, R: Request<T>> {
    pub data: Option<R::Response>,
    pub errors: Option<Vec<RawMeliorError>>,
}
