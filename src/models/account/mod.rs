mod badge;
mod effect;
mod gender;
mod info;
mod link;
mod requests;

pub use badge::Badge;
use chrono::{DateTime, Duration, Utc};
pub use effect::{Effect, EffectKind, EffectReasonKind};
pub use gender::Gender;
pub use info::*;
pub use link::Link;
use serde::{Deserialize, Serialize};

use crate::models::ImageRef;
use crate::{Client, Result};

/// The duration of an account's online status since when it was last online.
pub const ONLINE_DURATION: Duration = Duration::minutes(15);

/// Represents an account customization.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct AccountCustomization {
    /// The account's name color
    #[serde(rename = "nc")]
    pub name_color: Option<i32>,
    /// The account's active badge
    #[serde(rename = "ab")]
    pub active_badge: Option<Badge>,
}

/// Represents an account.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Account {
    /// A unique identifier of this account. Should always be set to a valid value if constructing
    /// with `{ ... }`
    #[serde(rename = "J_ID")]
    pub id: i64,
    /// The account's level
    #[serde(
        rename = "J_LVL",
        serialize_with = "crate::models::serialize_level",
        deserialize_with = "crate::models::deserialize_level"
    )]
    pub level: f32,
    /// The time when the account was last online
    #[serde(
        rename = "J_LAST_ONLINE_DATE",
        serialize_with = "crate::models::serialize_timestamp_millis",
        deserialize_with = "crate::models::deserialize_timestamp_millis"
    )]
    pub last_online_at: DateTime<Utc>,
    /// The account's name
    #[serde(rename = "J_NAME")]
    pub name: String,
    /// The account's avatar
    #[serde(
        serialize_with = "ImageRef::serialize_or_none",
        deserialize_with = "ImageRef::deserialize_or_none"
    )]
    pub avatar: Option<ImageRef>,
    /// The account's gender
    #[serde(rename = "sex")]
    pub gender: Gender,
    /// The account's karma in the last 30 days
    #[serde(
        serialize_with = "crate::models::serialize_level",
        deserialize_with = "crate::models::deserialize_level"
    )]
    pub karma30: f32,
    /// The amount that the account has donated
    #[serde(rename = "sponsor")]
    pub sponsor_amount: i64,
    /// The number of times this account has donated sequentially
    #[serde(rename = "sponsorTimes")]
    pub sponsor_count: i64,
    /// The account's effects
    #[serde(rename = "accountEffects")]
    pub effects: Vec<Effect>,
    /// The account's customization
    #[serde(rename = "czt")]
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
    /// Returns [client::RootServerError::Unavailable][crate::client::RootServerError::Unavailable]
    /// if there's no account with the provided identifier or [Error][crate::Error] if any other
    /// error occurred while sending the request.
    pub async fn get_by_id(client: &mut Client, id: i64) -> Result<Self> {
        Account::_get_account(client, Some(id), None).await
    }

    /// Get an account by its name. Doesn't require authentication.
    ///
    /// # Errors
    ///
    /// Returns [client::RootServerError::Unavailable][crate::client::RootServerError::Unavailable]
    /// if there's no account with the provided name or [Error][crate::Error] if any other error
    /// occurred while sending the request.
    pub async fn get_by_name(client: &mut Client, name: &str) -> Result<Self> {
        Account::_get_account(client, None, Some(name)).await
    }

    /// Search for accounts by their name.
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    pub async fn search(
        client: &mut Client,
        name: Option<&str>,
        offset: i64,
        follows_only: bool,
    ) -> Result<Vec<Self>> {
        Account::_search_accounts(client, name, offset, follows_only).await
    }

    /// Get [Info] about this account.
    ///
    /// # Errors
    ///
    /// Returns [client::RootServerError::Unavailable][crate::client::RootServerError::Unavailable]
    /// if there's no account with the contained identifier or [Error][crate::Error] if any other
    /// error occurred while sending the request.
    pub async fn info(&self, client: &mut Client) -> Result<Info> {
        Info::get_by_id(client, self.id).await
    }

    /// Get a list of accounts that are currently online (that were active less than
    /// [ONLINE_DURATION] ago).
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    pub async fn get_online(client: &mut Client, offset_date: DateTime<Utc>) -> Result<Vec<Self>> {
        Account::_get_online(client, offset_date).await
    }
}
