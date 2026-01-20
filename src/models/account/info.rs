use std::ops::Range;

use chrono::{DateTime, Utc};

use crate::client::Request as _;
use crate::models::{Gender, ImageRef, Link, Post, Publication};
use crate::requests::account::bio::{
    SetAgeRequest, SetDescriptionRequest, SetGenderRequest, SetStatusRequest,
};
use crate::requests::account::GetInfoRequest;
use crate::{Client, Result};

/// The number of links an account can contain.
pub const LINKS_COUNT: usize = 7;
/// The allowed range for an account age.
pub const AGE_RANGE: Range<i64> = 0..201;
/// The maximum allowed status length.
pub const STATUS_MAX_LENGTH: usize = 100;
/// The maximum allowed description length.
pub const DESCRIPTION_MAX_LENGTH: usize = 1000;

/// Represents information about an account.
#[derive(Default, Clone, Debug)]
pub struct Info {
    /// The date when this account was registered
    pub created_at: DateTime<Utc>,
    /// The date when this account's ban ends
    pub banned_until: Option<DateTime<Utc>>,
    /// A background inside this account's profile
    pub background: Option<ImageRef>,
    /// A GIF background inside this account's profile
    pub background_gif: Option<ImageRef>,
    /// Are you following this account?
    pub is_following: bool,
    /// Is this account following you?
    pub follows_me: bool,
    /// The number of users this account is followed to
    pub follows_count: u64,
    /// The number of users who are following this account
    pub followers_count: u64,
    /// A status which this account has set in their profile
    pub status: Option<String>,
    /// What age this account has given themselves?
    pub age: Option<i64>,
    /// A description (bio) of this account
    pub description: Option<String>,
    /// Links which this account has added to their profile
    pub links: Vec<Link>,
    /// Your note to this account
    pub note: Option<String>,
    /// A post which this account has pinned inside their profile
    pub pinned_post: Option<Publication<Post>>,
    /// How many times this account was banned?
    pub bans_count: u64,
    /// How many times this account was warned?
    pub warns_count: u64,
    /// Total karma earned by this account
    pub karma_total: f64,
    /// The number of rates placed by this account
    pub rates_count: u64,
    /// The sum of positive rates (karma of each rate is rounded to 1) placed by this account
    pub positive_rates_sum: i64,
    /// The sum of negative rates (karma of each rate is rounded to 1) placed by this account
    pub negative_rates_sum: i64,
    /// The number of fandoms this account can moderate
    pub moderating_fandoms_count: u64,
    /// The number of fandoms this account is subscribed to
    pub subscriptions_count: u64,
    /// The number of fandoms this account is viceroy in
    pub viceroys_count: u64,
    /// The number of stickers this account has added to their sticker collection
    pub stickers_count: u64,
    /// The number of users this account has blacklisted
    pub blacklisted_accounts_count: u64,
    /// The number of fandoms this account has blacklisted
    pub blacklisted_fandoms_count: u64,
}
impl Info {
    /// Get account information by its identifier.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::Unavailable`][crate::RootError::Unavailable] if there's no account
    /// with the provided identifier or [`Error`][crate::Error] if any other error occurred while
    /// sending the request.
    pub async fn get_by_id(client: &Client, id: u64) -> Result<Self> {
        GetInfoRequest::new_by_id(id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Get account information by its name.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::Unavailable`][crate::RootError::Unavailable] if there's no account
    /// with the provided name or [`Error`][crate::Error] if any other error occurred while sending
    /// the request.
    pub async fn get_by_name(client: &Client, name: &str) -> Result<Self> {
        GetInfoRequest::new_by_name(name)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Set your age. Must be within [`AGE_RANGE`]. Zero or `None` means no age.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::Other`][crate::RootError::Other] with the code `E_BAD_AGE` if the
    /// provided age is not within the range or [`Error`][crate::Error] if any other error occurred
    /// while sending the request.
    pub async fn set_age(client: &Client, age: Option<i64>) -> Result<()> {
        SetAgeRequest::new(age).send_request(client).await?;
        Ok(())
    }

    /// Set your status. Must be no longer than [`STATUS_MAX_LENGTH`]. Empty or `None` means no
    /// status.
    ///
    /// # Errors
    ///
    /// * [`RootError::AccessDenied`][crate::RootError::AccessDenied] if you aren't yet allowed to
    ///   change your status
    /// * [`RootRrror::Other`][crate::RootError::Other] with the code `E_BAD_SIZE` if the provided
    ///   status is longer than the maximum allowed length
    /// * [`Error`][crate::Error] if any other error occurred while sending the request.
    pub async fn set_status(client: &Client, status: Option<&str>) -> Result<()> {
        SetStatusRequest::new(status).send_request(client).await?;
        Ok(())
    }

    /// Set your description. Must be no longer than [`DESCRIPTION_MAX_LENGTH`]. Empty or `None`
    /// means no description.
    ///
    /// # Errors
    ///
    /// Returns [`RootError::Other`][crate::RootError::Other] with the code `E_BAD_SIZE` if the
    /// provided description is longer than the maximum allowed length or [`Error`][crate::Error]
    /// if any other error occurred while sending the request.
    pub async fn set_description(client: &Client, description: Option<&str>) -> Result<()> {
        SetDescriptionRequest::new(description)
            .send_request(client)
            .await?;
        Ok(())
    }

    /// Set your gender.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurred while sending the request.
    pub async fn set_gender(client: &Client, gender: Gender) -> Result<()> {
        SetGenderRequest::new(gender).send_request(client).await?;
        Ok(())
    }
}
