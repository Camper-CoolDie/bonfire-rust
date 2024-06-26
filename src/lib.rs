//! bonfire is an asynchronous client library for the [Bonfire](https://github.com/timas130/bonfire) API.
//! For now, there is only an interface to communicate with the server.
//!
//! # Example
//!
//! Creating a session to send a simple request to the real server and print the response.
//!
//! ```
//! use bonfire::session::RequestKind;
//! use bonfire::{Result, SecureConnector, Session};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let host = "cf2.bonfire.moe";
//!     let connector = SecureConnector::new(host, (host, 443));
//!     let object = json::object! { J_REQUEST_NAME: "RProjectVersionGet" };
//!
//!     let mut session = Session::builder()
//!         .kind(RequestKind::Bonfire)
//!         .connect(connector)
//!         .await?;
//!     let response = session.request("/", object).await?;
//!     println!("{}", response);
//!
//!     Ok(())
//! }
//! ```

/// Tools for communicating with the server using sessions.
pub mod session;
pub use session::Session;

/// Implementations for connecting to the server.
pub mod connector;
pub use connector::{Connector, InsecureConnector, SecureConnector};

/// Errors.
pub mod error;
pub use error::{Error, Result};
