//! bonfire is an asynchronous Rust client library for the [Bonfire](https://github.com/timas130/bonfire) API.
//!
//! - **Efficiency**. Fast asynchronous requests thanks to [hyper](https://crates.io/crates/hyper)
//! - **Simplicity**. Well-documented request-sending methods like
//!   [`Account::search()`][models::Account::search()]
//! - **Debuggability**. Requesting and errors are logged using [tracing](https://crates.io/crates/tracing)
//!
//! ## Example
//!
//! ```no_run
//! use std::fs;
//! use std::fs::File;
//! use std::io::Write;
//!
//! use anyhow::Result;
//! use bonfire::models::Auth;
//! use bonfire::Client;
//!
//! const EMAIL: &str = "user@example.com";
//! const PASSWORD: &str = "password";
//!
//! async fn save_credentials(client: &Client) -> Result<()> {
//!     let data = serde_json::to_string(&client.auth().await?)?;
//!     let mut file = File::create("credentials.json")?;
//!     file.write_all(data.as_bytes())?;
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Set up tracing
//!     tracing_subscriber::fmt::init();
//!
//!     // Build client & authenticate (either by using `credentials.json` or sending a login request)
//!     let auth_data = fs::read("credentials.json")
//!         .ok()
//!         .map(|data| serde_json::from_slice::<Auth>(&data))
//!         .transpose()?;
//!     let client = &match auth_data {
//!         Some(auth) => Client::builder().auth(auth).expect("invalid auth").build(),
//!         None => {
//!             let client = Client::default();
//!             client.login(EMAIL, PASSWORD).await?;
//!             save_credentials(&client).await?;
//!             client
//!         }
//!     };
//!
//!     // Get information about the currently authenicated user
//!     println!("{:#?}", Auth::get_me(client).await?);
//!
//!     // Save authentication credentials and exit program
//!     save_credentials(client).await?;
//!     Ok(())
//! }
//! ```
//!
//! The following dependencies are required for this example to work:
//!
//! ```toml
//! [dependencies]
//! anyhow = "1.0"
//! bonfire = "1.0"
//! serde_json = "1.0"
//! tokio = { version = "1.49", features = ["macros", "rt-multi-thread"] }
//! tracing-subscriber = "0.3"
//! ```

// General lints
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
// Clippy lints
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::struct_excessive_bools)]

/// Provides the core HTTP client implementation used for making requests across the library.
pub mod client;
/// Contains data structures, error types, and enums for constructing requests and parsing
/// responses.
pub mod models;
mod queries;
mod requests;

pub use client::{Client, ClientBuilder, Error, Result};
pub use queries::{MeliorError, QueryLocation, QueryPath};
pub(crate) use queries::{MeliorQuery, MeliorResponse};
pub use requests::{RootError, UnavailableError};
pub(crate) use requests::{RootRequest, RootResponse};

mod sealed {
    // Unreachable from outside the crate
    pub trait Sealed {}
}
