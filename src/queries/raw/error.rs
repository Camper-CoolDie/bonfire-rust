use serde::Deserialize;

use crate::{MeliorError, QueryLocation, QueryPath};

#[derive(Deserialize)]
pub(crate) struct RawMeliorError {
    pub message: String,
    pub locations: Option<Vec<RawQueryLocation>>,
    pub path: Option<Vec<RawQueryPath>>,
}

impl From<RawMeliorError> for MeliorError {
    fn from(value: RawMeliorError) -> Self {
        Self {
            message: value.message,
            locations: value
                .locations
                .map(|vec| vec.into_iter().map(Into::into).collect()),
            path: value
                .path
                .map(|vec| vec.into_iter().map(Into::into).collect()),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct RawQueryLocation {
    pub line: i32,
    pub column: i32,
}

impl From<RawQueryLocation> for QueryLocation {
    fn from(value: RawQueryLocation) -> Self {
        Self {
            line: value.line,
            column: value.column,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub(crate) enum RawQueryPath {
    Key(String),
    Index(i32),
}

impl From<RawQueryPath> for QueryPath {
    fn from(value: RawQueryPath) -> Self {
        match value {
            RawQueryPath::Key(key) => QueryPath::Key(key),
            RawQueryPath::Index(index) => QueryPath::Index(index),
        }
    }
}
