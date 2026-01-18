pub(crate) mod account;
mod error;
mod raw;

pub use error::{RootError, UnavailableError};
use serde::{Deserialize, Serialize};

use crate::requests::raw::RawRootError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RootRequest<'a, R> {
    #[serde(flatten)]
    pub content: R,
    #[serde(rename = "J_REQUEST_NAME")]
    pub request_name: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data_output: Vec<Option<u32>>,
    #[serde(rename = "J_API_ACCESS_TOKEN", skip_serializing_if = "Option::is_none")]
    pub api_access_token: Option<&'a str>,
    #[serde(rename = "requestApiVersion")]
    pub api_version: &'static str,
}

#[derive(Deserialize)]
#[serde(tag = "J_STATUS", content = "J_RESPONSE")]
pub(crate) enum RootResponse<S> {
    #[serde(rename = "J_STATUS_OK")]
    Ok(S),
    #[serde(rename = "J_STATUS_ERROR")]
    Error(RawRootError),
}
