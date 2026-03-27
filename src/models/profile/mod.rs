mod error;
mod gender;
mod link;

use std::ops::RangeInclusive;

use chrono::NaiveDate;
pub use error::*;
pub use gender::Gender;
pub use link::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::client::Request as _;
use crate::models::ImageRef;
use crate::queries::auth::GetProfileQuery;
use crate::queries::profile::SetBirthdayQuery;
use crate::requests::account::SetReferrerRequest;
use crate::requests::account::profile::{
    SetAgeRequest, SetAvatarRequest, SetBackgroundRequest, SetDescriptionRequest, SetNameRequest,
    SetStatusRequest,
};
use crate::{Client, Result};

/// The allowed range for an account's age.
pub const AGE_RANGE: RangeInclusive<i64> = 0..=200;

/// The maximum allowed length for an account's status message.
pub const STATUS_MAX_LENGTH: usize = 100;

/// The maximum allowed length for an account's description (bio).
pub const DESCRIPTION_MAX_LENGTH: usize = 1000;

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

/// A regular expression to validate a valid account name.
pub const NAME_REGEX: &str = r"^(?=.*[a-zA-Z])[a-zA-Z0-9]{3,25}$";
/// A regular expression to validate a name that has been unset (e.g., "User#1234").
pub const UNSET_NAME_REGEX: &str = r"^(?=[a-zA-Z0-9]*#)[a-zA-Z0-9#]+$";

/// Represents detailed information about the currently authenticated user's profile.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Profile {
    /// The unique identifier of your account
    pub id: u64,
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
impl Profile {
    /// Retrieves information about the currently authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Profile;
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// let profile = Profile::get(client).await?;
    /// println!("Logged in as {} (ID: {})", profile.name, profile.id);
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get(client: &Client) -> Result<Self> {
        GetProfileQuery::new()
            .send_request(client)
            .await?
            .try_into()
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
    /// # use bonfire::models::Profile;
    /// # use bonfire::{Client, Result};
    /// use chrono::NaiveDate;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// let birthday = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    /// Profile::set_birthday(client, birthday).await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn set_birthday(client: &Client, birthday: NaiveDate) -> Result<Self> {
        SetBirthdayQuery::new(birthday)
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

    /// Sets the provided account as the referrer for the currently logged-in account.
    ///
    /// # Errors
    ///
    /// * Returns [`SetReferrerError::AlreadySet`][crate::models::account::SetReferrerError::AlreadySet]
    ///   if the referrer has already been set.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn set_referrer(client: &Client, id: u64) -> Result<()> {
        SetReferrerRequest::new(id).send_request(client).await?;
        Ok(())
    }

    /// Sets the authenticated user's display name.
    ///
    /// The new name must adhere to the pattern defined by [`NAME_REGEX`]. If the name has been
    /// cleared by administrators, it can be set again. To check if the current name can be changed,
    /// use [`UNSET_NAME_REGEX`].
    ///
    /// # Errors
    ///
    /// * Returns [`SetNameError::AlreadySet`] if the name has already been set.
    /// * Returns [`SetNameError::InvalidName`] if the provided name does not match [`NAME_REGEX`].
    /// * Returns [`SetNameError::Taken`] if the provided name is already in use by another account.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn set_name(client: &Client, name: &str) -> Result<()> {
        SetNameRequest::new(name).send_request(client).await?;
        Ok(())
    }
}
