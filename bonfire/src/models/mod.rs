/// Contains structs and helper types related to user accounts.
pub mod account;
/// Contains structs, errors, and helper types for authentication processes.
pub mod auth;
/// Contains structs and helper types related to chats.
pub mod chat;
/// Contains common, reusable models and types used across the API.
pub mod common;
/// Contains structs and helper types related to fandoms.
pub mod fandom;
/// Contains structs and helper types for notifications.
pub mod notification;
mod other;
/// Contains structs and helper types related to user profiles.
pub mod profile;
/// Contains structs, kind enums, and helper types for publications.
pub mod publication;
/// Contains structs and helper types for user settings.
pub mod settings;
mod streams;

// TODO: Fandom/Account.get_by_ref

pub use account::{
    AccessLevel, Account, AccountRef, Badge, Effect, Info as AccountInfo, Stat as AccountStat,
};
pub use auth::Auth;
pub use chat::{AnyChat, Chat, Direct, FandomRoot, FandomSub, Group, Tag as ChatTag};
pub use common::{Category, ImageRef, Language, VoiceRef};
pub use fandom::{Fandom, FandomRef};
pub use other::{Config, InitialData};
pub use profile::{Gender, Link, Profile};
pub use publication::{AnyPublication, ChatMessage, Comment, Post, PostTag, Publication, Reaction};
pub use settings::Settings;
