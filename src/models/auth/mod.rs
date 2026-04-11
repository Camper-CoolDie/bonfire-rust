mod error;

pub use error::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::client::Request as _;
use crate::models::Profile;
use crate::queries::auth::GetProfileQuery;
use crate::{Client, Result};

/// Represents authentication credentials for a user session.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Auth {
    /// The access token for the authenticated session
    pub access_token: String,
    /// The refresh token used to obtain new access tokens
    pub refresh_token: String,
}
impl Auth {
    /// Retrieves information about the currently authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Auth;
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// #     let client = &Client::default();
    /// let profile = Auth::get_profile(client).await?;
    /// println!("Logged in as {} (ID: {})", profile.name, profile.id);
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn get_profile(client: &Client) -> Result<Profile> {
        GetProfileQuery::new()
            .send_request(client)
            .await?
            .try_into()
    }
}
