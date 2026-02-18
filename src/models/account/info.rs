use std::ops::RangeInclusive;

use chrono::{DateTime, Utc};

use crate::client::Request as _;
use crate::models::{Gender, ImageRef, Link, Post, Publication};
use crate::requests::account::profile::{
    SetAgeRequest, SetAvatarRequest, SetBackgroundRequest, SetDescriptionRequest, SetGenderRequest,
    SetLinkRequest, SetStatusRequest,
};
use crate::requests::account::GetInfoRequest;
use crate::{Client, Result};

/// The allowed range for an account's age.
pub const AGE_RANGE: RangeInclusive<i64> = 0..=200;

/// The maximum allowed length for an account's status message.
pub const STATUS_MAX_LENGTH: usize = 100;

/// The maximum allowed length for an account's description (bio).
pub const DESCRIPTION_MAX_LENGTH: usize = 1000;

/// The maximum number of links an account's profile can contain.
pub const LINKS_MAX_COUNT: usize = 7;
/// The maximum allowed length for a link's title.
pub const LINK_TITLE_MAX_LENGTH: usize = 30;
/// The maximum allowed length for a link's URI.
pub const LINK_URI_MAX_LENGTH: usize = 500;

/// The maximum allowed size in bytes for a static avatar image.
pub const AVATAR_MAX_SIZE: usize = 32 * 1024;
/// The maximum allowed dimension (width or height) for a static avatar image.
pub const AVATAR_MAX_DIMENSION: usize = 384;
/// The maximum allowed size in bytes for a GIF avatar image.
pub const AVATAR_GIF_MAX_SIZE: usize = 256 * 1024;
/// The maximum allowed dimension (width or height) for a GIF avatar image.
pub const AVATAR_GIF_MAX_DIMENSION: usize = 92;

/// The maximum allowed size in bytes for a static background image.
pub const BACKGROUND_MAX_SIZE: usize = 256 * 1024;
/// The maximum allowed width for a static background image.
pub const BACKGROUND_MAX_WIDTH: usize = 1200;
/// The maximum allowed height for a static background image.
pub const BACKGROUND_MAX_HEIGHT: usize = 600;
/// The maximum allowed size in bytes for a GIF background image.
pub const BACKGROUND_GIF_MAX_SIZE: usize = 2 * 1024 * 1024;
/// The maximum allowed width for a GIF background image.
pub const BACKGROUND_GIF_MAX_WIDTH: usize = 400;
/// The maximum allowed height for a GIF background image.
pub const BACKGROUND_GIF_MAX_HEIGHT: usize = 200;

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
    pub moderating_fandoms_count: u64,
    /// The number of fandoms this account is subscribed to
    pub subscriptions_count: u64,
    /// The number of fandoms this account serves as a viceroy in
    pub viceroys_count: u64,
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

    /// Sets the account's age.
    ///
    /// The age must be within the [`AGE_RANGE`]. A value of `0` or `None` indicates no age is set.
    ///
    /// # Errors
    ///
    /// Returns [`SetAgeError::InvalidAge`][super::SetAgeError::InvalidAge] if the provided age is
    /// outside the allowed range, or [`Error`][crate::Error] if any other error occurs during the
    /// request.
    pub async fn set_age(client: &Client, age: Option<i64>) -> Result<()> {
        SetAgeRequest::new(age).send_request(client).await?;
        Ok(())
    }

    /// Sets the account's status message.
    ///
    /// The status must not exceed [`STATUS_MAX_LENGTH`]. An empty string or `None` clears the
    /// status.
    ///
    /// # Errors
    ///
    /// * Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if the user is not yet
    ///   permitted to change their status.
    /// * Returns [`SetProfileTextError::TooLong`][super::SetProfileTextError::TooLong] if the
    ///   provided status exceeds the maximum allowed length.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn set_status(client: &Client, status: Option<&str>) -> Result<()> {
        SetStatusRequest::new(status).send_request(client).await?;
        Ok(())
    }

    /// Sets the account's description (bio).
    ///
    /// The description must not exceed [`DESCRIPTION_MAX_LENGTH`]. An empty string or `None` clears
    /// the description.
    ///
    /// # Errors
    ///
    /// Returns [`SetProfileTextError::TooLong`][super::SetProfileTextError::TooLong] if the
    /// provided description exceeds the maximum allowed length, or [`Error`][crate::Error] if any
    /// other error occurs during the request.
    pub async fn set_description(client: &Client, description: Option<&str>) -> Result<()> {
        SetDescriptionRequest::new(description)
            .send_request(client)
            .await?;
        Ok(())
    }

    /// Sets the account's declared gender.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn set_gender(client: &Client, gender: Gender) -> Result<()> {
        SetGenderRequest::new(gender).send_request(client).await?;
        Ok(())
    }

    /// Sets an external link in the account's profile.
    ///
    /// The link is identified by its `index` (0-based, up to [`LINKS_MAX_COUNT`] - 1). The `title`
    /// must not exceed [`LINK_TITLE_MAX_LENGTH`] and the `uri` must not exceed
    /// [`LINK_URI_MAX_LENGTH`]. To delete a link at a specific index, both `title` and `uri` must
    /// be provided as empty strings.
    ///
    /// # Errors
    ///
    /// * Returns [`SetLinkError::TitleTooLong`][super::SetLinkError::TitleTooLong] if the provided
    ///   title exceeds the maximum allowed length.
    /// * Returns [`SetLinkError::UriTooLong`][super::SetLinkError::UriTooLong] if the provided URI
    ///   exceeds the maximum allowed length.
    /// * Returns [`Error::UnsuccessfulResponse`][crate::Error::UnsuccessfulResponse] with the
    ///   status `500` if the provided `index` is greater than or equal to [`LINKS_MAX_COUNT`].
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn set_link(client: &Client, index: u32, title: &str, uri: &str) -> Result<()> {
        SetLinkRequest::new(index, title, uri)
            .send_request(client)
            .await?;
        Ok(())
    }

    /// Removes an external link from the account's profile at the specified index.
    ///
    /// # Errors
    ///
    /// Returns [`Error::UnsuccessfulResponse`][crate::Error::UnsuccessfulResponse] with the status
    /// `500` if the provided `index` is greater than or equal to [`LINKS_MAX_COUNT`], or
    /// [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn remove_link(client: &Client, index: u32) -> Result<()> {
        SetLinkRequest::new(index, "", "")
            .send_request(client)
            .await?;
        Ok(())
    }

    /// Sets the account's avatar.
    ///
    /// This method accepts both static and GIF images. The server automatically determines the
    /// appropriate avatar type based on the provided image data.
    ///
    /// Static avatars cannot exceed [`AVATAR_MAX_SIZE`] in size, and their dimensions must be no
    /// larger than [`AVATAR_MAX_DIMENSION`]. GIF avatars cannot exceed [`AVATAR_GIF_MAX_SIZE`] in
    /// size, and their dimensions must be no larger than [`AVATAR_GIF_MAX_DIMENSION`].
    ///
    /// # Errors
    ///
    /// * Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if the user is not yet
    ///   permitted to upload a GIF avatar.
    /// * Returns [`SetProfileImageError::SizeExceeded`][super::SetProfileImageError::SizeExceeded]
    ///   if the image file size is too large.
    /// * Returns [`SetProfileImageError::DimensionsTooHigh`][super::SetProfileImageError::DimensionsTooHigh]
    ///   if the image dimensions are too large.
    /// * Returns [`Error::UnsuccessfulResponse`][crate::Error::UnsuccessfulResponse] with the
    ///   status `500` if the provided avatar is not a valid image or GIF.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn set_avatar(client: &Client, avatar: &[u8]) -> Result<ImageRef> {
        Ok(SetAvatarRequest::new(avatar)
            .send_request(client)
            .await?
            .into())
    }

    /// Sets the account's static profile background.
    ///
    /// Static background images cannot exceed [`BACKGROUND_MAX_SIZE`] in size, and their dimensions
    /// must be no larger than [`BACKGROUND_MAX_WIDTH`] in width and [`BACKGROUND_MAX_HEIGHT`] in
    /// height.
    ///
    /// # Errors
    ///
    /// * Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if the user is not yet
    ///   permitted to change the static background image.
    /// * Returns [`SetProfileImageError::SizeExceeded`][super::SetProfileImageError::SizeExceeded]
    ///   if the image file size is too large.
    /// * Returns [`SetProfileImageError::DimensionsTooHigh`][super::SetProfileImageError::DimensionsTooHigh]
    ///   if the image dimensions are too large.
    /// * Returns [`Error::UnsuccessfulResponse`][crate::Error::UnsuccessfulResponse] with the
    ///   status `500` if the provided background is not a valid image.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn set_background(client: &Client, background: &[u8]) -> Result<ImageRef> {
        Ok(SetBackgroundRequest::new(background)
            .send_request(client)
            .await?
            .into())
    }

    /// Sets the account's animated (GIF) profile background, returning a tuple where the first
    /// [`ImageRef`] is for the just uploaded `first_frame`, and the second one is for the GIF
    /// itself.
    ///
    /// GIF background images cannot exceed [`BACKGROUND_GIF_MAX_SIZE`] in size, and their
    /// dimensions must be no larger than [`BACKGROUND_GIF_MAX_WIDTH`] in width and
    /// [`BACKGROUND_GIF_MAX_HEIGHT`] in height.
    ///
    /// # Errors
    ///
    /// * Returns [`RootError::AccessDenied`][crate::RootError::AccessDenied] if the user is not yet
    ///   permitted to change the animated background image.
    /// * Returns [`SetProfileImageError::SizeExceeded`][super::SetProfileImageError::SizeExceeded]
    ///   if the image file size is too large.
    /// * Returns [`SetProfileImageError::DimensionsTooHigh`][super::SetProfileImageError::DimensionsTooHigh]
    ///   if the image dimensions are too large.
    /// * Returns [`Error::UnsuccessfulResponse`][crate::Error::UnsuccessfulResponse] with the
    ///   status `500` if the provided background GIF is not a valid image.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn set_background_gif(
        client: &Client,
        first_frame: &[u8],
        animated: &[u8],
    ) -> Result<(ImageRef, ImageRef)> {
        SetBackgroundRequest::new_gif(first_frame, animated)
            .send_request(client)
            .await?
            .try_into()
    }
}
