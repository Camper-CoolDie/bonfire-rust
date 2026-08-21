#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Profile;
use crate::requests::account::profile::SetLinkRequest;
use crate::{Client, Result};

/// Represents an external link displayed within an account's profile.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Link {
    /// The 0-based index of this link
    pub index: u32,
    /// The title or display text for this link
    pub title: String,
    /// The URI this link points to
    pub uri: String,
}

impl Profile {
    /// The maximum number of links an account's profile can contain.
    pub const LINKS_MAX_COUNT: usize = 7;
    /// The maximum allowed length for a link's title.
    pub const LINK_TITLE_MAX_LENGTH: usize = 30;
    /// The maximum allowed length for a link's URI.
    pub const LINK_URI_MAX_LENGTH: usize = 500;

    /// Sets an external link in the account's profile.
    ///
    /// The link is identified by its `index` (0-based, up to
    /// [`LINKS_MAX_COUNT`][Self::LINKS_MAX_COUNT] - 1). The `title` must not exceed
    /// [`LINK_TITLE_MAX_LENGTH`][Self::LINK_TITLE_MAX_LENGTH] and the `uri` must not exceed
    /// [`LINK_URI_MAX_LENGTH`][Self::LINK_URI_MAX_LENGTH]. To delete a link at a specific index,
    /// both `title` and `uri` must be provided as empty strings.
    ///
    /// # Errors
    ///
    /// * Returns [`SetLinkError::TitleTooLong`][super::SetLinkError::TitleTooLong] if the provided
    ///   title exceeds the maximum allowed length.
    /// * Returns [`SetLinkError::UriTooLong`][super::SetLinkError::UriTooLong] if the provided URI
    ///   exceeds the maximum allowed length.
    /// * Returns [`Error::UnsuccessfulResponse`][crate::Error::UnsuccessfulResponse] with the
    ///   status `500` if the provided `index` is greater than or equal to
    ///   [`LINKS_MAX_COUNT`][Self::LINKS_MAX_COUNT].
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
    /// `500` if the provided `index` is greater than or equal to
    /// [`LINKS_MAX_COUNT`][Self::LINKS_MAX_COUNT], or [`Error`][crate::Error] if any other error
    /// occurs during the request.
    pub async fn remove_link(client: &Client, index: u32) -> Result<()> {
        SetLinkRequest::new(index, "", "")
            .send_request(client)
            .await?;
        Ok(())
    }
}
