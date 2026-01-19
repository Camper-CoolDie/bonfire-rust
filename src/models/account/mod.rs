mod badge;
mod effect;
mod gender;
mod info;
mod link;

pub use badge::Badge;
use chrono::{DateTime, Duration, Utc};
pub use effect::{Effect, EffectKind, EffectReasonKind};
pub use gender::Gender;
pub use info::*;
pub use link::Link;

use crate::client::Request;
use crate::models::ImageRef;
use crate::requests::account::{
    GetAccountRequest, GetInfoRequest, GetOnlineRequest, SearchAccountsRequest,
};
use crate::{Client, Result};

/// The duration of an account's online status since when it was last online.
pub const ONLINE_DURATION: Duration = Duration::minutes(15);

/// Represents an account customization.
#[derive(Default, Clone, Debug)]
pub struct AccountCustomization {
    /// The account's name color
    pub name_color: Option<u32>,
    /// The account's active badge
    pub active_badge: Option<Badge>,
}

/// Represents an account.
#[derive(Default, Clone, Debug)]
pub struct Account {
    /// A unique identifier of this account. Should always be set to a valid value if constructing
    /// with `{ ... }`
    pub id: u64,
    /// The account's level
    pub level: f32,
    /// The time when the account was last online
    pub last_online_at: DateTime<Utc>,
    /// The account's name
    pub name: String,
    /// The account's avatar
    pub avatar: Option<ImageRef>,
    /// The account's gender
    pub gender: Gender,
    /// The account's karma in the last 30 days
    pub karma30: f32,
    /// The amount that the account has donated
    pub sponsor_amount: u64,
    /// The number of times this account has donated sequentially
    pub sponsor_count: u64,
    /// The account's effects
    pub effects: Vec<Effect>,
    /// The account's customization
    pub customization: AccountCustomization,
}
impl Account {
    /// Check if this account is currently online.
    pub fn is_online(&self) -> bool {
        Utc::now() - self.last_online_at < ONLINE_DURATION
    }

    /// Get an account by its identifier. Doesn't require authentication.
    ///
    /// # Errors
    ///
    /// Returns [RootError::Unavailable][crate::models::RootError::Unavailable] if there's no
    /// account with the provided identifier or [Error][crate::Error] if any other error occurred
    /// while sending the request.
    pub async fn get_by_id(client: &Client, id: u64) -> Result<Self> {
        GetAccountRequest::new_by_id(id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Get an account by its name. Doesn't require authentication.
    ///
    /// # Errors
    ///
    /// Returns [RootError::Unavailable][crate::models::RootError::Unavailable] if there's no
    /// account with the provided name or [Error][crate::Error] if any other error occurred while
    /// sending the request.
    pub async fn get_by_name(client: &Client, name: &str) -> Result<Self> {
        GetAccountRequest::new_by_name(name)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Search for accounts by their name.
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    pub async fn search(
        client: &Client,
        name: Option<&str>,
        offset: u64,
        follows_only: bool,
    ) -> Result<Vec<Self>> {
        SearchAccountsRequest::new(name, offset, follows_only)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Get [Info] about this account.
    ///
    /// # Errors
    ///
    /// Returns [RootError::Unavailable][crate::models::RootError::Unavailable] if there's no
    /// account with the contained identifier or [Error][crate::Error] if any other error occurred
    /// while sending the request.
    pub async fn info(&self, client: &Client) -> Result<Info> {
        GetInfoRequest::new_by_id(self.id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Get a list of accounts that are currently online (that were active less than
    /// [ONLINE_DURATION] ago).
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    pub async fn get_online(
        client: &Client,
        offset_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Self>> {
        GetOnlineRequest::new(offset_date)
            .send_request(client)
            .await?
            .try_into()
    }
}
