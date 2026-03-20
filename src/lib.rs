//! bonfire-rust is a modern, well-documented asynchronous client library for the
//! [Bonfire API](https://github.com/timas130/bonfire).
//!
//! ## Features
//!
//! - **Modern asynchronous design**: Built on [`tokio`](https://crates.io/crates/tokio) and [`hyper`](https://crates.io/crates/hyper)
//!   for non-blocking, high-performance network requests.
//! - **Ergonomic and fluent API**: Interact with API models like [`Account`][models::Account] and
//!   [`Fandom`][models::Fandom] directly (e.g.,
//!   [`account.get_follows()`][models::Account::get_follows()]) rather than managing request
//!   structs manually.
//! - **Automatic session management**: Handles authentication token refreshing transparently. Log
//!   in once and the client manages the rest.
//!
//! ## Usage
//!
//! ### Quick Start: Logging in and printing user's email and ID
//!
//! This simple example shows how to create a client, log in and fetch the profile of the currently
//! authenticated user:
//!
//! ```no_run
//! use bonfire::prelude::*;
//!
//! const EMAIL: &str = "user@example.com";
//! const PASSWORD: &str = "password";
//!
//! #[tokio::main]
//! async fn main() -> ApiResult<()> {
//!     // Create a client with default settings and log in. Use `Client::builder` to edit these
//!     // settings
//!     let client = &Client::default();
//!     client.login(EMAIL, PASSWORD).await?;
//!
//!     // Fetch the authenticated user's profile
//!     let profile = Profile::get(client).await?;
//!     println!("Logged in as {} (ID: {})", profile.name, profile.id);
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Advanced usage: Searching accounts by name
//!
//! Some methods like [`Account::search`][models::Account::search] return a
//! [`Stream`][futures::Stream], allowing you to asynchronously iterate over accounts while the API
//! loads them. This example also shows how to properly save and load authentication tokens to reuse
//! the same session:
//!
//! ```no_run
//! use std::fs;
//! use std::fs::File;
//! use std::io::Write;
//!
//! use anyhow::Result;
//! use bonfire::prelude::*;
//! use futures_util::TryStreamExt as _;
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
//!     // Fetch accounts that have "Sus" in their name
//!     Account::search(client, Some("Sus"), false, 0)
//!         .try_for_each(|account| async move {
//!             println!("User {} (ID: {})", account.name, account.id);
//!             Ok(())
//!         })
//!         .await?;
//!
//!     // Save authentication credentials and exit program
//!     save_credentials(client).await?;
//!     Ok(())
//! }
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
/// A set of exports which can be helpful to use.
pub mod prelude;
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
