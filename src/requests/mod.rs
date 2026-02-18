pub(crate) mod account;
mod error;
mod raw;

use std::result::Result as StdResult;

pub use error::{RootError, UnavailableError};
use serde::{Deserialize, Serialize, Serializer};

use crate::client::Request;
pub(crate) use crate::requests::raw::RawRootError;

fn serialize_data_output<S: Serializer>(
    value: &[Option<i32>],
    serializer: S,
) -> StdResult<S::Ok, S::Error> {
    let result = value
        .iter()
        .map(|option| match option {
            Some(value) => *value,
            None => -1,
        })
        .collect::<Vec<i32>>();

    result.serialize(serializer)
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RootRequest<'a, R: Request> {
    #[serde(flatten)]
    pub content: &'a R,
    #[serde(rename = "J_REQUEST_NAME")]
    pub request_name: &'static str,
    #[serde(
        serialize_with = "serialize_data_output",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub data_output: Vec<Option<i32>>,
    #[serde(rename = "J_API_ACCESS_TOKEN", skip_serializing_if = "Option::is_none")]
    pub api_access_token: Option<&'a str>,
    #[serde(rename = "requestApiVersion")]
    pub api_version: &'static str,
}

#[derive(Deserialize)]
#[serde(tag = "J_STATUS", content = "J_RESPONSE")]
pub(crate) enum RootResponse<R: Request> {
    #[serde(rename = "J_STATUS_OK")]
    Ok(R::Response),
    #[serde(rename = "J_STATUS_ERROR")]
    Error(RawRootError),
}
