mod error;

pub use error::{Error, TfaKind, TfaRequired};
use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Me;
use crate::queries::auth::{LoginEmailQuery, LogoutQuery, MeQuery, RefreshQuery};
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
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Auth;
    /// # use bonfire::Result;
    /// use bonfire::Client;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = Client::default();
    /// client.login("email", "password").await?;
    ///
    /// let me = Auth::me(&client).await.unwrap();
    /// println!("Logged in as {}", me.name);
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn me(client: &Client) -> Result<Me> {
        MeQuery::new().send_request(client).await
    }

    pub(crate) async fn login(client: &Client, email: &str, password: &str) -> Result<Self> {
        LoginEmailQuery::new(email, password)
            .send_request(client)
            .await
    }

    pub(crate) async fn logout(client: &Client) -> Result<()> {
        LogoutQuery::new().send_request(client).await?;
        Ok(())
    }

    pub(crate) async fn refresh(&self, client: &Client) -> Result<Self> {
        RefreshQuery::new(&self.access_token)
            .send_request(client)
            .await
    }
}
