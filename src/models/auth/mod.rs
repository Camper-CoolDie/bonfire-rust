mod error;

pub use error::{Error, TfaKind, TfaRequired};
use serde::{Deserialize, Serialize};

use crate::client::Request as _;
use crate::models::Me;
use crate::queries::auth::MeQuery;
use crate::{Client, Result};

/// Represents an authentication session.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Auth {
    /// The session's access token
    pub access_token: String,
    /// The session's refresh token
    pub refresh_token: String,
}
impl Auth {
    /// Get information about the currently authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurred while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Auth;
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = Client::default();
    /// let me = Auth::get_me(&client).await?;
    /// println!("Logged in as {}", me.name);
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get_me(client: &Client) -> Result<Me> {
        Ok(MeQuery::new().send_request(client).await?.into())
    }
}
