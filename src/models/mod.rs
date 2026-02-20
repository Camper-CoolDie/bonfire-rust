/// Contains structs and helper types related to user accounts.
pub mod account;
/// Contains structs, errors, and helper types for authentication processes.
pub mod auth;
/// Contains common, reusable models and types used across the API.
pub mod common;
/// Contains structs and helper types related to fandoms.
pub mod fandom;
/// Contains structs and helper types related to user profiles.
pub mod profile;
/// Contains structs, kind enums, and helper types for publications.
pub mod publication;

pub use account::{Account, Badge, Effect, Gender, Info as AccountInfo, Link, Stat as AccountStat};
pub use auth::Auth;
pub use common::{Category, ImageRef, Language};
pub use fandom::Fandom;
pub use profile::Me;
pub use publication::{AnyPublication, Post, Publication, Reaction};
