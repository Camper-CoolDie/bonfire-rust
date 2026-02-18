use std::convert::Infallible;
use std::error::Error as StdError;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::{Client, MeliorError, Result, RootError};

// All request-specific errors have to define one of the two methods depending on their type. The
// panic can occur when Request::Error and Request itself have different types
#[allow(unused_variables)]
pub(crate) trait RequestError: StdError + Send + Sync + 'static {
    fn try_from_root(error: &RootError) -> Result<Option<Self>>
    where
        Self: Sized,
    {
        panic!("tried to convert RootError into RequestError (wrong error type?)")
    }

    fn try_from_melior(error: &MeliorError) -> Result<Option<Self>>
    where
        Self: Sized,
    {
        panic!("tried to convert MeliorError into RequestError (wrong error type?)")
    }
}

pub(crate) trait Request: Serialize {
    type Response: DeserializeOwned;
    type Error: RequestError;

    async fn send_request(&self, client: &Client) -> Result<Self::Response>;
}

// Requests that don't return anything should use this struct as the Response type
#[derive(Deserialize)]
pub(crate) struct EmptyResponse {}

// Requests that don't have a request-specific error should use Infallible as the Error type
impl RequestError for Infallible {
    fn try_from_root(_error: &RootError) -> Result<Option<Self>> {
        Ok(None)
    }

    fn try_from_melior(_error: &MeliorError) -> Result<Option<Self>> {
        Ok(None)
    }
}
