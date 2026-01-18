use std::ops::Range;

use chrono::{DateTime, Utc};

use crate::models::{Gender, ImageRef, Link, Post, Publication};
use crate::requests::account::bio::{
    SetAgeRequest, SetDescriptionRequest, SetGenderRequest, SetStatusRequest,
};
use crate::requests::account::GetInfoRequest;
use crate::{Client, Request, Result};

/// The number of links an account can contain.
pub const LINKS_COUNT: usize = 7;
/// The allowed range for an age.
pub const AGE_RANGE: Range<i64> = 0..201;
/// The maximum allowed status length.
pub const STATUS_MAX_LENGTH: usize = 100;
/// The maximum allowed description length.
pub const DESCRIPTION_MAX_LENGTH: usize = 1000;

/// Represents information about an account.
#[derive(Default, Clone, Debug)]
pub struct Info {
    /// The date when this account was created
    pub created_at: DateTime<Utc>,
    /// The date when this account's ban ends
    pub banned_until: Option<DateTime<Utc>>,
    /// The account's background
    pub background: Option<ImageRef>,
    /// The account's GIF background
    pub background_gif: Option<ImageRef>,
    /// Are you following this account?
    pub is_following: bool,
    /// Is this account following you?
    pub follows_me: bool,
    /// The number of users this account is followed to
    pub follows_count: i64,
    /// The number of users who are following this account
    pub followers_count: i64,
    /// The account's status
    pub status: Option<String>,
    /// The account's age
    pub age: Option<i64>,
    /// The account's description
    pub description: Option<String>,
    /// The account's links
    pub links: Vec<Link>,
    /// Your note to this account
    pub note: Option<String>,
    /// A post which this account has pinned inside their profile
    pub pinned_post: Option<Publication<Post>>,
    /// The account's bans count
    pub bans_count: i64,
    /// The account's warns count
    pub warns_count: i64,
    /// The account's total karma
    pub karma_total: f32,
    /// The number of rates placed by this account
    pub rates_count: i64,
    /// The sum of this account's positive rates (each rate's amount is rounded to 1)
    pub positive_rates_sum: i64,
    /// The sum of this account's negative rates (each rate's amount is rounded to 1)
    pub negative_rates_sum: i64,
    /// The number of fandoms this account can moderate
    pub moderating_fandoms_count: i64,
    /// The number of fandoms this account subscribed to
    pub subscriptions_count: i64,
    /// The number of fandoms this account is viceroy in
    pub viceroys_count: i64,
    /// The number of stickers this account has added to their collection
    pub stickers_count: i64,
    /// The number of users this account has blacklisted
    pub blacklisted_accounts_count: i64,
    /// The number of fandoms this account has blacklisted
    pub blacklisted_fandoms_count: i64,
}
impl Info {
    /// Get account information by its identifier.
    ///
    /// # Errors
    ///
    /// Returns [RootError::Unavailable][crate::models::RootError::Unavailable] if there's no
    /// account with the provided identifier or [Error][crate::Error] if any other error occurred
    /// while sending the request.
    pub async fn get_by_id(client: &Client, id: i64) -> Result<Self> {
        GetInfoRequest::new_by_id(id).send_request(client).await
    }

    /// Get account information by its name.
    ///
    /// # Errors
    ///
    /// Returns [RootError::Unavailable][crate::models::RootError::Unavailable] if there's no
    /// account with the provided name or [Error][crate::Error] if any other error occurred while
    /// sending the request.
    pub async fn get_by_name(client: &Client, name: &str) -> Result<Self> {
        GetInfoRequest::new_by_name(name).send_request(client).await
    }

    /// Set your age. Must be within [AGE_RANGE]. Zero or `None` means no age.
    ///
    /// # Errors
    ///
    /// Returns [RootError::Other][crate::models::RootError::Other] with the code `E_BAD_AGE` if
    /// the provided age is not within the range or [Error][crate::Error] if any other error
    /// occurred while sending the request.
    pub async fn set_age(client: &Client, age: Option<i64>) -> Result<()> {
        SetAgeRequest::new(age.unwrap_or(0))
            .send_request(client)
            .await
    }

    /// Set your status. Must be no longer than [STATUS_MAX_LENGTH]. Empty or `None` means no
    /// status.
    ///
    /// # Errors
    ///
    /// * [RootError::AccessDenied][crate::models::RootError::AccessDenied] if you aren't yet
    ///   allowed to change your status
    /// * [RootRrror::Other][crate::models::RootError::Other] with the code `E_BAD_SIZE` if the
    ///   provided status is longer than the maximum allowed length
    /// * [Error][crate::Error] if any other error occurred while sending the request.
    pub async fn set_status(client: &Client, status: Option<&str>) -> Result<()> {
        SetStatusRequest::new(status.unwrap_or(""))
            .send_request(client)
            .await
    }

    /// Set your description. Must be no longer than [DESCRIPTION_MAX_LENGTH]. Empty or `None`
    /// means no description.
    ///
    /// # Errors
    ///
    /// Returns [RootError::Other][crate::models::RootError::Other] with the code `E_BAD_SIZE` if
    /// the provided description is longer than the maximum allowed length or [Error][crate::Error]
    /// if any other error occurred while sending the request.
    pub async fn set_description(client: &Client, description: Option<&str>) -> Result<()> {
        SetDescriptionRequest::new(description.unwrap_or(""))
            .send_request(client)
            .await
    }

    /// Set your gender.
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    pub async fn set_gender(client: &Client, gender: Gender) -> Result<()> {
        SetGenderRequest::new(gender).send_request(client).await
    }
}
