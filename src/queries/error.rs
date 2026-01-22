use std::error::Error;
use std::fmt;

/// Represents an error returned by the Melior (GraphQL) server.
#[derive(Debug)]
pub struct MeliorError {
    /// The error message
    pub message: String,
    /// The locations within the query that caused the error
    pub locations: Option<Vec<QueryLocation>>,
    /// The path within the query that caused the error
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
                    QueryPath::Key(key) => write!(f, ".{key}")?,
                    QueryPath::Index(index) => write!(f, "[{index}]")?,
                }
            }
            write!(f, ")")?;
        }
        Ok(())
    }
}

impl Error for MeliorError {}

/// Represents a specific location within a GraphQL query.
#[derive(Debug)]
pub struct QueryLocation {
    /// The line number in the query
    pub line: i32,
    /// The column number in the query
    pub column: i32,
}

/// Represents a single part of a query's path, used to pinpoint the exact location of an error.
#[derive(Debug)]
pub enum QueryPath {
    /// A key within an object
    Key(String),
    /// An index within an array
    Index(i32),
}
