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

use crate::client::Request as _;
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
    /// A color of this account's name
    pub name_color: Option<u32>,
    /// A badge which this account has picked as main
    pub active_badge: Option<Badge>,
}

/// Represents an account.
#[derive(Default, Clone, Debug)]
pub struct Account {
    /// A unique identifier of this account. Should always be set to a valid value if constructing
    /// with `{ ... }`
    pub id: u64,
    /// A level of this account
    pub level: f64,
    /// The time when this account was last online
    pub last_online_at: DateTime<Utc>,
    /// A name of this account
    pub name: String,
    /// An avatar of this account
    pub avatar: Option<ImageRef>,
    /// A gender of this account
    pub gender: Gender,
    /// Karma earned by this account in the last 30 days
    pub karma30: f64,
    /// How much money the account has donated
    pub sponsor_amount: u64,
    /// The number of sequential times this account has donated
    pub sponsor_count: u64,
    /// Effects which this account currently holds
    pub effects: Vec<Effect>,
    /// A customization of this account
    pub customization: AccountCustomization,
}
impl Account {
    /// Create a new `Account` with only its identifier set. Useful when you don't need other
    /// fields but need to send an associated request, however using a struct obtained from
    /// [`Account::get_by_id()`] or [`Account::get_by_name()`] is preferable.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Account;
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = Client::default();
    /// let account = Account::new(1234);
    /// println!("{:#?}", account.get_info(&client).await?);
    /// #    Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }

    /// Check if this account is currently online. Requires [`Account::last_online_at`] to be set.
    #[must_use]
    pub fn is_online(&self) -> bool {
        Utc::now() - self.last_online_at < ONLINE_DURATION
    }

    /// Get an account by its identifier. Doesn't require authentication.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::Unavailable`][crate::RootError::Unavailable] if there's no account
    /// with the provided identifier or [`Error`][crate::Error] if any other error occurred while
    /// sending the request.
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
    /// Returns [`RootError::Unavailable`][crate::RootError::Unavailable] if there's no account
    /// with the provided name or [`Error`][crate::Error] if any other error occurred while sending
    /// the request.
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
    /// Returns [`Error`][crate::Error] if an error occurred while sending the request.
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

    /// Get [`Info`] about this account.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::Unavailable`][crate::RootError::Unavailable] if there's no account
    /// with the contained identifier or [`Error`][crate::Error] if any other error occurred while
    /// sending the request.
    pub async fn get_info(&self, client: &Client) -> Result<Info> {
        GetInfoRequest::new_by_id(self.id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Get a list of accounts that are currently online (that were active less than
    /// [`ONLINE_DURATION`] ago).
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurred while sending the request.
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
