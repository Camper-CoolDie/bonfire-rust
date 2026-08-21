#![warn(missing_docs)]

pub mod client;
mod listener;
pub mod models;
pub mod prelude;
mod requests;

#[expect(warnings)]
mod proto {
    include!(concat!(env!("OUT_DIR"), "/mcs_proto.rs"));
    include!(concat!(env!("OUT_DIR"), "/checkin_proto.rs"));
}

pub use client::{Builder as ClientBuilder, Client, Error, Result};
use client::{Connection, HyperClient};
use listener::Listener;
