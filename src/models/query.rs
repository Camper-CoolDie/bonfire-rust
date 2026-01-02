use std::error::Error;
use std::fmt;

/// Represents an error from the melior server.
#[derive(Debug)]
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
#[derive(Debug)]
pub struct QueryLocation {
    /// Line number in the query
    pub line: i32,
    /// Column number in the query
    pub column: i32,
}

/// Represents a part of a query path.
#[derive(Debug)]
pub enum QueryPath {
    /// A key inside an object
    Key(String),
    /// An index inside an array
    Index(i32),
}
