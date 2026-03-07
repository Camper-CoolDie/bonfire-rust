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
use futures::Stream;
pub use gender::Gender;
pub use info::Info;
pub use link::Link;
pub use prison::PrisonEntry;
pub use stat::Stat;

use crate::client::Request as _;
use crate::models::streams::{paginated_by_date_stream, paginated_stream};
use crate::models::{Fandom, ImageRef};
use crate::requests::account::blocklist::{
    BLOCKED_ACCOUNTS_PAGE_SIZE, BlockAccountRequest, CheckAccountBlockedRequest,
    GetBlockedAccountsRequest, UnblockAccountRequest,
};
use crate::requests::account::profile::{
    CURATED_FANDOMS_PAGE_SIZE, ChangeFollowRequest, FOLLOWS_PAGE_SIZE, GetCuratedFandomsRequest,
    GetFollowsRequest, GetModeratedFandomsRequest, GetSubscriptionsRequest,
    MODERATED_FANDOMS_PAGE_SIZE, SUBSCRIPTIONS_PAGE_SIZE,
};
use crate::requests::account::{
    ACCOUNTS_SEARCH_PAGE_SIZE, GetAccountRequest, GetInfoRequest, GetOnlineRequest,
    GetPrisonRequest, GetStatRequest, ONLINE_PAGE_SIZE, PRISON_PAGE_SIZE, ReportRequest,
    SearchAccountsRequest,
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
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn search<'a>(
        client: &'a Client,
        name: Option<&'a str>,
        follows_only: bool,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        paginated_stream(
            move |offset| async move {
                SearchAccountsRequest::new(name, offset, follows_only)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            0,
            ACCOUNTS_SEARCH_PAGE_SIZE,
        )
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
    /// this method returns a default, empty [`Stat`].
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get_stat(&self, client: &Client) -> Result<Stat> {
        Ok(GetStatRequest::new(self.id)
            .send_request(client)
            .await?
            .into())
    }

    /// Retrieves a [`Stream`] of accounts that are currently online.
    ///
    /// An account is considered online if it was active less than [`ONLINE_DURATION`] ago. The
    /// stream handles pagination automatically, fetching new pages of results as needed. The
    /// resulting stream is sorted by [`Account::last_online_at`] in ascending order, meaning the
    /// oldest online accounts are yielded first.
    ///
    /// It is recommended to use [`try_collect`][futures::TryStreamExt::try_collect] to gather all
    /// results into a [`Vec`] if a complete and consistent list of online users is needed. Delaying
    /// requests by processing items one by one may cause some accounts to be missed due to their
    /// online status expiring between page fetches.
    ///
    /// If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream will yield
    /// that single error and then terminate.
    pub fn get_online(client: &Client) -> impl Stream<Item = Result<Self>> + '_ {
        let limit_date = Utc::now();

        paginated_by_date_stream(
            move |offset_date| async move {
                GetOnlineRequest::new(offset_date, limit_date)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            None,
            ONLINE_PAGE_SIZE,
            |last_account| last_account.last_online_at,
        )
    }

    /// Retrieves a [`Stream`] of all currently banned accounts.
    ///
    /// This method returns a [`Stream`] that yields individual [`PrisonEntry`] instances as they
    /// are retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_prison(client: &Client) -> impl Stream<Item = Result<PrisonEntry>> + '_ {
        paginated_stream(
            move |offset| async move {
                GetPrisonRequest::new(offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            0,
            PRISON_PAGE_SIZE,
        )
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

    /// Retrieves a [`Stream`] of accounts this account is following.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_follows<'a>(&'a self, client: &'a Client) -> impl Stream<Item = Result<Self>> + 'a {
        paginated_stream(
            move |offset| async move {
                GetFollowsRequest::new_follows(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            0,
            FOLLOWS_PAGE_SIZE,
        )
    }

    /// Retrieves a [`Stream`] of accounts that are following this account.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_followers<'a>(
        &'a self,
        client: &'a Client,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        paginated_stream(
            move |offset| async move {
                GetFollowsRequest::new_followers(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            0,
            FOLLOWS_PAGE_SIZE,
        )
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

    /// Retrieves a [`Stream`] of accounts that are currently blocked by this account.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_blocked_accounts<'a>(
        &'a self,
        client: &'a Client,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        paginated_stream(
            move |offset| async move {
                GetBlockedAccountsRequest::new(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            0,
            BLOCKED_ACCOUNTS_PAGE_SIZE,
        )
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
    /// Returns [`ReportError::AlreadyReported`] if this account has already been reported, or
    /// [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn report(&self, client: &Client, comment: &str) -> Result<&Self> {
        ReportRequest::new(self.id, comment)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Retrieves a [`Stream`] of fandoms this account is subscribed to.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_subscriptions<'a>(
        &'a self,
        client: &'a Client,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        paginated_stream(
            move |offset| async move {
                GetSubscriptionsRequest::new(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            0,
            SUBSCRIPTIONS_PAGE_SIZE,
        )
    }

    /// Retrieves a [`Stream`] of fandoms this account moderates.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_moderated_fandoms<'a>(
        &'a self,
        client: &'a Client,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        paginated_stream(
            move |offset| async move {
                GetModeratedFandomsRequest::new(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            0,
            MODERATED_FANDOMS_PAGE_SIZE,
        )
    }

    /// Retrieves a [`Stream`] of fandoms this account curates.
    ///
    /// This method returns a [`Stream`] that yields individual [`Fandom`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn get_curated_fandoms<'a>(
        &'a self,
        client: &'a Client,
    ) -> impl Stream<Item = Result<Fandom>> + 'a {
        paginated_stream(
            move |offset| async move {
                GetCuratedFandomsRequest::new(self.id, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            0,
            CURATED_FANDOMS_PAGE_SIZE,
        )
    }
}
