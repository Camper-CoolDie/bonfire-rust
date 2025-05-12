#![warn(missing_docs)]

mod client;
mod connector;
pub mod model;

pub use client::{
    Client, ClientBuilder, Error, MeliorError, QueryLocation, QueryPath, Result, RootError,
};
pub use connector::Error as ConnectorError;
pub(crate) use connector::{Connector, ConnectorWrapper};
