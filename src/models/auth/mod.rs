mod error;

pub use error::*;
use serde::{Deserialize, Serialize};

use crate::client::Request as _;
use crate::models::Me;
use crate::queries::auth::MeQuery;
use crate::{Client, Result};

/// Represents authentication credentials for a user session.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
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
    /// # let client = &Client::default();
    /// let me = Auth::get_me(client).await?;
    /// println!("Logged in as {}", me.name);
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get_me(client: &Client) -> Result<Me> {
        Ok(MeQuery::new().send_request(client).await?.into())
    }
}
