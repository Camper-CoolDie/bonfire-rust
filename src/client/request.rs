use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RootError {
    pub code: String,
    pub message_error: String,
    pub params: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Request<R> {
    #[serde(flatten)]
    pub content: R,
    #[serde(rename = "J_REQUEST_NAME")]
    pub request_name: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub data_output: Vec<Option<u32>>,
    #[serde(
        rename = "J_API_ACCESS_TOKEN",
        skip_serializing_if = "String::is_empty"
    )]
    pub api_access_token: String,
    #[serde(rename = "J_API_BOT_TOKEN", skip_serializing_if = "String::is_empty")]
    pub api_bot_token: String,
}

#[derive(Deserialize)]
#[serde(tag = "J_STATUS", content = "J_RESPONSE")]
pub(super) enum RootResponse<S> {
    #[serde(rename = "J_STATUS_OK")]
    Ok(S),
    #[serde(rename = "J_STATUS_ERROR")]
    Error(RootError),
}
