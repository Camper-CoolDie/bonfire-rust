use chrono::{DateTime, Utc};

use crate::client::Request as _;
use crate::models::{ImageRef, Link, Post, Publication};
use crate::requests::account::GetInfoRequest;
use crate::{Client, Result};

/// Represents detailed information about an account's profile.
#[derive(Default, Clone, Debug)]
pub struct Info {
    /// The date when this account was registered
    pub created_at: DateTime<Utc>,
    /// The date when this account's ban is scheduled to end
    pub banned_until: Option<DateTime<Utc>>,
    /// The background image set for this account's profile
    pub background: Option<ImageRef>,
    /// The GIF background image set for this account's profile
    pub background_gif: Option<ImageRef>,
    /// Indicates if you are currently following this account
    pub is_following: bool,
    /// Indicates if this account is currently following you
    pub follows_me: bool,
    /// The total number of users this account is following
    pub follows_count: u64,
    /// The total number of users who are following this account
    pub followers_count: u64,
    /// The custom status message set by this account in their profile
    pub status: Option<String>,
    /// The age provided by this account, if set
    pub age: Option<i64>,
    /// The description (bio) provided by this account
    pub description: Option<String>,
    /// External links added by this account to their profile, always ordered by [`Link::index`]
    /// but may contain gaps in indices
    pub links: Vec<Link>,
    /// Your private note associated with this account
    pub note: Option<String>,
    /// A specific post that this account has pinned to their profile
    pub pinned_post: Option<Publication<Post>>,
    /// The total number of times this account has been banned
    pub bans_count: u64,
    /// The total number of times this account has received a warning
    pub warns_count: u64,
    /// The total karma earned by this account
    pub karma_total: f64,
    /// The total number of rates placed by this account
    pub rates_count: u64,
    /// The sum of all positive rates (each rounded to 1) placed by this account
    pub positive_rates_sum: i64,
    /// The sum of all negative rates (each rounded to 1) placed by this account
    pub negative_rates_sum: i64,
    /// The number of fandoms this account has moderation privileges in
    pub moderated_fandoms_count: u64,
    /// The number of fandoms this account is curating
    pub curated_fandoms_count: u64,
    /// The number of fandoms this account is subscribed to
    pub subscriptions_count: u64,
    /// The number of stickers this account has added to their collection
    pub stickers_count: u64,
    /// The number of users this account has blocked
    pub blocked_accounts_count: u64,
    /// The number of fandoms this account has blocked
    pub blocked_fandoms_count: u64,
}
impl Info {
    /// Retrieves detailed account information by its unique identifier.
    ///
    /// # Errors
    ///
    /// Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if no account with
    /// the provided identifier exists, or [`Error`][crate::Error] if any other error occurs during
    /// the request.
    pub async fn get_by_id(client: &Client, id: u64) -> Result<Self> {
        GetInfoRequest::new_by_id(id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves detailed account information by its name.
    ///
    /// # Errors
    ///
    /// Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if no account with
    /// the provided name exists, or [`Error`][crate::Error] if any other error occurs during the
    /// request.
    pub async fn get_by_name(client: &Client, name: &str) -> Result<Self> {
        GetInfoRequest::new_by_name(name)
            .send_request(client)
            .await?
            .try_into()
    }
}
