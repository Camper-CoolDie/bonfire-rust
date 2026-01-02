/// Account implementation and helper structs.
pub mod account;
/// Authentication implementation, errors and helper structs.
pub mod auth;
/// Common, shared models used across the API.
pub mod common;
/// Fandom implementations and helper structs.
pub mod fandom;
/// Profile implementation and helper structs.
pub mod profile;
/// Publication implementations, kinds and helper structs.
pub mod publication;
pub(crate) mod query;
pub(crate) mod raw;
pub(crate) mod request;

pub use account::{Account, Badge, Effect, Gender, Info as AccountInfo, Link};
pub use auth::Auth;
pub use common::{Category, ImageRef, Language};
pub use fandom::Fandom;
pub use profile::Me;
pub use publication::{AnyPublication, Post, Publication, Reaction};
pub use query::{MeliorError, QueryLocation, QueryPath};
pub(crate) use raw::{MeliorResponse, Query, Request, RootResponse};
pub use request::{RootError, UnavailableError};
use serde::Deserialize;

#[derive(Deserialize)]
struct EmptyResponse {}
