//! bonfire is an asynchronous client library for the [Bonfire](https://github.com/timas130/bonfire) API.
//! For now, there is only an interface to communicate with the server.
//!
//! # Example
//!
//! Creating a session to send a simple request to the real server and print the response.
//!
//! ```
//! use std::net::{SocketAddr, IpAddr, Ipv4Addr};
//! use bonfire::Session;
//! use bonfire::session::{Result, RequestKind, SecureConnector};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(116, 202, 162, 215)), 443);
//!     let host = "cf2.bonfire.moe";
//!     let connector = SecureConnector::new(host, addr);
//!     let object = json::object!{ J_REQUEST_NAME: "RProjectVersionGet" };
//!
//!     let mut session = Session::builder()
//!         .kind(RequestKind::Bonfire)
//!         .connect(connector).await?;
//!     let response = session.request("/", object).await?;
//!     println!("{}", response);
//!
//!     Ok(())
//! }
//! ```

/// Tools for communicating with the server using sessions.
pub mod session;
pub use session::Session;
