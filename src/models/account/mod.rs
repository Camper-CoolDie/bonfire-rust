mod badge;
mod blocklist;
mod customization;
mod effect;
mod error;
mod info;
mod prison;
mod profile;
mod stat;

pub use badge::Badge;
use chrono::{DateTime, Duration, Utc};
pub use customization::Customization;
pub use effect::{Effect, Kind as EffectKind, ReasonKind as EffectReasonKind};
pub use error::*;
use futures::Stream;
pub use info::Info;
pub use prison::PrisonEntry;
pub use stat::Stat;

use crate::client::Request as _;
use crate::models::streams::{auto_paginated_stream, paginated_stream};
use crate::models::{Gender, ImageRef};
use crate::requests::account::{
    GetAccountRequest, GetOnlineRequest, GetPrisonRequest, ReportRequest, SearchAccountsRequest,
};
use crate::{Client, Result};

/// The maximum duration an account can be offline while still considered "online".
pub const ONLINE_DURATION: Duration = Duration::minutes(15);

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
    pub customization: Customization,
}
impl Account {
    /// Creates a new `Account` instance with only its identifier set.
    ///
    /// This is useful when you only need to reference an account by its ID for sending associated
    /// requests. However, obtaining a fully populated `Account` struct from methods like
    /// [`Account::by_id()`] or [`Account::by_name()`] is generally preferred.
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
    /// println!("{:#?}", account.info(client).await?);
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
    pub async fn by_id(client: &Client, id: u64) -> Result<Self> {
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
    pub async fn by_name(client: &Client, name: &str) -> Result<Self> {
        GetAccountRequest::new_by_name(name)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Searches for accounts by their name.
    ///
    /// This method returns a [`Stream`] that yields individual [`Account`] instances as they are
    /// retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of accounts from the beginning
    /// of the list.
    ///
    /// If `query` is `None` or empty, this method returns a list of accounts the currently
    /// logged-in user is following. If the user is not following any accounts, it will return a
    /// list of online users.
    ///
    /// If an [`Error`][crate::Error] occurs during the retrieval of any page, the stream
    /// will yield that single error and then terminate.
    pub fn search<'a>(
        client: &'a Client,
        query: Option<&'a str>,
        offset: usize,
    ) -> impl Stream<Item = Result<Self>> + 'a {
        auto_paginated_stream(
            move |offset| async move {
                SearchAccountsRequest::new(query, offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            SearchAccountsRequest::PAGE_SIZE,
        )
    }

    /// Retrieves a [`Stream`] of accounts that are currently online.
    ///
    /// An account is considered online if it was active less than [`ONLINE_DURATION`] ago. The
    /// `offset_date` parameter can be used to skip accounts online before a specific time. The
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
    pub fn online(
        client: &Client,
        offset_date: Option<DateTime<Utc>>,
    ) -> impl Stream<Item = Result<Self>> + '_ {
        let limit_date = Utc::now();

        paginated_stream(
            move |offset_date| async move {
                GetOnlineRequest::new(offset_date, limit_date)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset_date,
            |accounts, _| {
                let length = accounts.len();
                (length >= GetOnlineRequest::PAGE_SIZE)
                    .then(|| accounts.last())
                    .flatten()
                    .map(|account| Some(account.last_online_at))
            },
        )
    }

    /// Retrieves a [`Stream`] of all currently banned accounts.
    ///
    /// This method returns a [`Stream`] that yields individual [`PrisonEntry`] instances as they
    /// are retrieved. The stream handles pagination automatically, fetching new pages of results as
    /// needed. The `offset` parameter can be used to skip a number of banned accounts from the
    /// beginning of the list. If an [`Error`][crate::Error] occurs during the retrieval of any
    /// page, the stream will yield that single error and then terminate.
    pub fn prison(client: &Client, offset: usize) -> impl Stream<Item = Result<PrisonEntry>> + '_ {
        auto_paginated_stream(
            move |offset| async move {
                GetPrisonRequest::new(offset)
                    .send_request(client)
                    .await?
                    .try_into()
            },
            offset,
            GetPrisonRequest::PAGE_SIZE,
        )
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
}
