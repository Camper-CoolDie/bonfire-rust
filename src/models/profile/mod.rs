mod error;

use std::ops::RangeInclusive;

use chrono::NaiveDate;
pub use error::*;

use crate::client::Request as _;
use crate::models::{Gender, ImageRef};
use crate::queries::auth::MeQuery;
use crate::queries::profile::SetBirthdayQuery;
use crate::requests::account::profile::{
    SetAgeRequest, SetAvatarRequest, SetBackgroundRequest, SetDescriptionRequest, SetGenderRequest,
    SetLinkRequest, SetStatusRequest,
};
use crate::requests::account::SetReferrerRequest;
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

/// Represents detailed information about the currently authenticated user's profile.
#[derive(Default, Clone, Debug)]
pub struct Me {
    /// The unique identifier of your account (not guaranteed to be an integer)
    pub id: String,
    /// Your display name
    pub name: String,
    /// Your registered email address
    pub email: String,
    /// Your last known level, cached by the API
    pub cached_level: f64,
    /// Your date of birth, or `None` if not set
    pub birthday: Option<NaiveDate>,
    /// Indicates whether you are allowed to view NSFW content, or `None` if `birthday` is also
    /// `None`
    pub is_nsfw_allowed: Option<bool>,
}
impl Me {
    /// Retrieves information about the currently authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Me;
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// let me = Me::get(client).await?;
    /// println!("Logged in as {}", me.name);
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get(client: &Client) -> Result<Self> {
        Ok(MeQuery::new().send_request(client).await?.into())
    }

    /// Sets the authenticated user's birthday.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Me;
    /// # use bonfire::{Client, Result};
    /// use chrono::NaiveDate;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// let birthday = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    /// Me::set_birthday(client, birthday).await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn set_birthday(client: &Client, birthday: NaiveDate) -> Result<Me> {
        Ok(SetBirthdayQuery::new(birthday)
            .send_request(client)
            .await?
            .into())
    }

    /// Sets the account's age.
    ///
    /// The age must be within the [`AGE_RANGE`]. A value of `0` or `None` indicates no age is set.
    ///
    /// # Errors
    ///
    /// Returns [`SetAgeError::InvalidAge`] if the provided age is outside the allowed range, or
    /// [`Error`][crate::Error] if any other error occurs during the request.
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
    /// * Returns [`SetProfileTextError::TooLong`] if the provided status exceeds the maximum
    ///   allowed length.
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
    /// Returns [`SetProfileTextError::TooLong`] if the provided description exceeds the maximum
    /// allowed length, or [`Error`][crate::Error] if any other error occurs during the request.
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
    /// * Returns [`SetLinkError::TitleTooLong`] if the provided title exceeds the maximum allowed
    ///   length.
    /// * Returns [`SetLinkError::UriTooLong`] if the provided URI exceeds the maximum allowed
    ///   length.
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
    /// * Returns [`SetProfileImageError::SizeExceeded`] if the image file size is too large.
    /// * Returns [`SetProfileImageError::DimensionsTooHigh`] if the image dimensions are too large.
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
    /// * Returns [`SetProfileImageError::SizeExceeded`] if the image file size is too large.
    /// * Returns [`SetProfileImageError::DimensionsTooHigh`] if the image dimensions are too large.
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
    /// * Returns [`SetProfileImageError::SizeExceeded`] if the image file size is too large.
    /// * Returns [`SetProfileImageError::DimensionsTooHigh`] if the image dimensions are too large.
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

    /// Sets the prrovided account as the referrer for the currently logged-in account.
    ///
    /// # Errors
    ///
    /// * Returns [`SetReferrerError::AlreadySet`][crate::models::account::SetReferrerError::AlreadySet]
    ///   if the referrer has already been set.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn set_referrer(&self, client: &Client, id: u64) -> Result<&Self> {
        SetReferrerRequest::new(id).send_request(client).await?;
        Ok(self)
    }
}
