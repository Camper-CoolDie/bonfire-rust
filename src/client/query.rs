use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

/// Represents an error from the melior server.
#[derive(Clone, Debug, Deserialize)]
pub struct MeliorError {
    /// Message of the error
    pub message: String,
    /// Query locations that caused the error
    pub locations: Option<Vec<QueryLocation>>,
    /// Part of a query path that caused the error
    pub path: Option<Vec<QueryPath>>,
}

impl fmt::Display for MeliorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if let Some(locations) = &self.locations {
            write!(f, " (at")?;
            for location in locations {
                write!(f, " {}:{}", location.line, location.column)?;
            }
            write!(f, ")")?;
        }
        if let Some(path) = &self.path {
            write!(f, " (in ")?;
            for part in path {
                match part {
                    QueryPath::Key(key) => write!(f, ".{}", key)?,
                    QueryPath::Index(index) => write!(f, "[{}]", index)?,
                }
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl Error for MeliorError {}

/// Represents a location inside a query.
#[derive(Clone, Debug, Deserialize)]
pub struct QueryLocation {
    /// Line number in the query
    pub line: i32,
    /// Column number in the query
    pub column: i32,
}

/// Represents a part of a query path.
#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum QueryPath {
    /// A key inside an object
    Key(String),
    /// An index inside an array
    Index(i32),
}

#[derive(Serialize)]
pub(crate) struct Query<R> {
    pub variables: R,
    pub query: &'static str,
}

#[derive(Deserialize)]
pub(super) struct MeliorResponse<S> {
    pub data: Option<S>,
    pub errors: Option<Vec<MeliorError>>,
}
