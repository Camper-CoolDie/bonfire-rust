//! bonfire is an asynchronous Rust client library for the
//! [Bonfire](https://github.com/timas130/bonfire) API.
//!
//! - **Efficiency**. Fast asynchronous requests using [hyper](https://crates.io/crates/hyper)
//! - **Simplicity**. Well-documented request-sending methods like
//!   [Account::search()][models::Account::search()]
//! - **Debuggability**. Connection attempts, requests and errors are logged using [tracing](https://crates.io/crates/tracing)
//!
//! ## Example
//!
//! ```no_run
//! use std::fs;
//! use std::fs::File;
//! use std::io::Write;
//!
//! use bonfire::models::Auth;
//! use bonfire::{Client, Result};
//!
//! const EMAIL: &str = "user@example.com";
//! const PASSWORD: &str = "password";
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     // Set up tracing
//!     tracing_subscriber::fmt::init();
//!
//!     // Authenticate
//!     let mut client = Client::connect().await?;
//!     if let Ok(ref data) = fs::read("credential.json") {
//!         client.auth = Some(serde_json::from_slice(&data)?);
//!     } else {
//!         Auth::login(&mut client, EMAIL, PASSWORD).await?;
//!     }
//!
//!     // Get information about the currently authenicated user
//!     println!("{:#?}", Auth::info(&mut client).await?);
//!
//!     // Save auth tokens to "credential.json"
//!     if let Some(ref auth) = client.auth.as_ref() {
//!         let data = serde_json::to_string(auth)?;
//!         let mut file = File::create("credential.json").unwrap();
//!         file.write_all(data.as_bytes()).unwrap();
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! Required dependencies in `Cargo.toml` for this example:
//!
//! ```toml
//! [dependencies]
//! bonfire = "1.0"
//! serde_json = "1.0"
//! tokio = { version = "1.48", features = ["macros", "rt-multi-thread"] }
//! tracing-subscriber = "0.3"
//! ```
#![warn(missing_docs)]

/// Client implementation, builder, errors and result.
pub mod client;
/// Connector errors and result.
pub mod connector;
/// Structs and enums for making requests.
pub mod models;

pub use client::{Client, ClientBuilder, Error, Result};
pub(crate) use connector::{Connector, ConnectorWrapper};
