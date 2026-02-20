mod badge;
mod effect;
mod error;
mod gender;
mod info;
mod link;
mod prison;
mod stat;

pub use badge::Badge;
use chrono::{DateTime, Duration, Utc};
pub use effect::{Effect, EffectKind, EffectReasonKind};
pub use error::*;
pub use gender::Gender;
pub use info::Info;
pub use link::Link;
pub use prison::PrisonEntry;
pub use stat::Stat;

use crate::client::Request as _;
use crate::models::{Fandom, ImageRef};
use crate::requests::account::blocklist::{
    BlockAccountRequest, CheckAccountBlockedRequest, GetBlockedAccountsRequest,
    UnblockAccountRequest,
};
use crate::requests::account::profile::{
    ChangeFollowRequest, GetCuratedFandomsRequest, GetFollowsRequest, GetModeratedFandomsRequest,
    GetSubscriptionsRequest,
};
use crate::requests::account::{
    GetAccountRequest, GetInfoRequest, GetOnlineRequest, GetPrisonRequest, GetStatRequest,
    ReportRequest, SearchAccountsRequest,
};
use crate::requests::fandom::blocklist::GetBlockedFandomIdsRequest;
use crate::{Client, Result};

/// The maximum duration an account can be offline while still considered "online".
pub const ONLINE_DURATION: Duration = Duration::minutes(15);

/// Represents customizable aspects of an account's appearance.
#[derive(Default, Clone, Debug)]
pub struct AccountCustomization {
    /// The hexadecimal color code for this account's name (e.g., `0xFFFFFF`).
    pub name_color: Option<u32>,
    /// The badge currently selected and displayed for this account.
    pub active_badge: Option<Badge>,
}

/// Represents a Bonfire user account.
#[derive(Default, Clone, Debug)]
pub struct Account {
    /// The unique identifier of this account
    pub id: u64,
    /// The current level of this account
    pub level: f64,
    /// The timestamp when this account was last detected online
    pub last_online_at: DateTime<Utc>,
    /// The display name of this account
    pub name: String,
    /// The avatar image of this account, if set
    pub avatar: Option<ImageRef>,
    /// The declared gender of this account
    pub gender: Gender,
    /// The karma earned by this account over the last 30 days
    pub karma30: f64,
    /// The amount of money this account has donated sequentially
    pub sponsor_amount: u64,
    /// The number of consecutive times this account has donated
    pub sponsor_count: u64,
    /// A list of effects currently applied to this account
    pub effects: Vec<Effect>,
    /// Customization settings applied to this account's appearance
    pub customization: AccountCustomization,
}
impl Account {
    /// Creates a new `Account` instance with only its identifier set.
    ///
    /// This is useful when you only need to reference an account by its ID for sending associated
    /// requests. However, obtaining a fully populated `Account` struct from methods like
    /// [`Account::get_by_id()`] or [`Account::get_by_name()`] is generally preferred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Account;
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// let account = Account::new(1234);
    /// println!("{:#?}", account.get_info(client).await?);
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

    /// Checks if this account is currently considered online.
    ///
    /// This method relies on the [`Account::last_online_at`] field being set.
    #[must_use]
    pub fn is_online(&self) -> bool {
        Utc::now() - self.last_online_at < ONLINE_DURATION
    }

    /// Retrieves an account by its unique identifier.
    ///
    /// This method does not require authentication.
    ///
    /// # Errors
    ///
    /// Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if no account with
    /// the provided identifier exists, or [`Error`][crate::Error] if any other error occurs during
    /// the request.
    pub async fn get_by_id(client: &Client, id: u64) -> Result<Self> {
        GetAccountRequest::new_by_id(id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves an account by its name.
    ///
    /// This method does not require authentication.
    ///
    /// # Errors
    ///
    /// Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if no account with
    /// the provided name exists, or [`Error`][crate::Error] if any other error occurs during the
    /// request.
    pub async fn get_by_name(client: &Client, name: &str) -> Result<Self> {
        GetAccountRequest::new_by_name(name)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Searches for accounts by their name.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
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

    /// Retrieves detailed [`Info`] about this account.
    ///
    /// # Errors
    ///
    /// Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if no account with
    /// the contained identifier exists, or [`Error`][crate::Error] if any other error occurs during
    /// the request.
    pub async fn get_info(&self, client: &Client) -> Result<Info> {
        GetInfoRequest::new_by_id(self.id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves this account's statistics. If no account with the contained identifier exists,
    /// this method returns a default, empty `Stat`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn get_stat(&self, client: &Client) -> Result<Stat> {
        Ok(GetStatRequest::new(self.id)
            .send_request(client)
            .await?
            .into())
    }

    /// Retrieves a list of accounts that are currently online.
    ///
    /// An account is considered online if it was active less than [`ONLINE_DURATION`] ago.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_online(
        client: &Client,
        offset_date: Option<DateTime<Utc>>,
    ) -> Result<Vec<Self>> {
        GetOnlineRequest::new(offset_date)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves a list of all currently banned accounts.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_prison(client: &Client, offset: u64) -> Result<Vec<PrisonEntry>> {
        GetPrisonRequest::new(offset)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Follows this account.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you attempt to follow
    /// your own account, or [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn follow(&self, client: &Client) -> Result<&Self> {
        ChangeFollowRequest::new_follow(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Unfollows this account.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you attempt to
    /// unfollow your own account, or [`Error`][crate::Error] if any other error occurs during the
    /// request.
    pub async fn unfollow(&self, client: &Client) -> Result<&Self> {
        ChangeFollowRequest::new_unfollow(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Retrieves a list of accounts this account is following.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_follows(&self, client: &Client, offset: u64) -> Result<Vec<Self>> {
        GetFollowsRequest::new_follows(self.id, offset)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves a list of accounts that are following this account.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_followers(&self, client: &Client, offset: u64) -> Result<Vec<Self>> {
        GetFollowsRequest::new_followers(self.id, offset)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Blocks this account, hiding all its publications and disallowing direct messages from it.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you attempt to block
    /// your own account, or [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn block(&self, client: &Client) -> Result<&Self> {
        BlockAccountRequest::new(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Unblocks this account, making its publications reappear and allowing it to send you direct
    /// messages.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn unblock(&self, client: &Client) -> Result<&Self> {
        UnblockAccountRequest::new(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Checks if this account is currently blocked by you.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you attempt to check
    /// the block status of your own account, or [`Error`][crate::Error] if any other error occurs
    /// during the request.
    pub async fn check_blocked(&self, client: &Client) -> Result<bool> {
        Ok(CheckAccountBlockedRequest::new(self.id)
            .send_request(client)
            .await?
            .into())
    }

    /// Retrieves a list of accounts that are currently blocked by this account.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_blocked_accounts(&self, client: &Client, offset: u64) -> Result<Vec<Self>> {
        GetBlockedAccountsRequest::new(self.id, offset)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves a list of IDs of fandoms that are currently blocked by this account.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_blocked_fandom_ids(&self, client: &Client) -> Result<Vec<u64>> {
        Ok(GetBlockedFandomIdsRequest::new(self.id)
            .send_request(client)
            .await?
            .into())
    }

    /// Reports this account.
    ///
    /// # Errors
    ///
    /// * Returns [`ReportError::AlreadyReported`][crate::models::account::ReportError::AlreadyReported]
    ///   if this account has already been reported.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn report(&self, client: &Client, comment: &str) -> Result<&Self> {
        ReportRequest::new(self.id, comment)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Retrieves a list of fandoms this account is subscribed to.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_subscriptions(&self, client: &Client, offset: u64) -> Result<Vec<Fandom>> {
        GetSubscriptionsRequest::new(self.id, offset)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves a list of fandoms this account moderates.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_moderated_fandoms(&self, client: &Client, offset: u64) -> Result<Vec<Fandom>> {
        GetModeratedFandomsRequest::new(self.id, offset)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves a list of fandoms this account curates.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_curated_fandoms(&self, client: &Client, offset: u64) -> Result<Vec<Fandom>> {
        GetCuratedFandomsRequest::new(self.id, offset)
            .send_request(client)
            .await?
            .try_into()
    }
}
