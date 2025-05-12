use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MeliorError {
    pub message: String,
    pub locations: Option<Vec<QueryLocation>>,
    pub path: Option<Vec<QueryPath>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct QueryLocation {
    pub line: i32,
    pub column: i32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum QueryPath {
    Key(String),
    Index(i32),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Query<R> {
    pub variables: R,
    pub query: &'static str,
}

#[derive(Deserialize)]
pub(super) struct MeliorResponse<S> {
    pub data: Option<S>,
    pub errors: Option<Vec<MeliorError>>,
}
