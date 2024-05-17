use crate::error::Error;

/// Result type returned from methods that can have `Error`s.
pub type Result<T> = std::result::Result<T, Error>;
