mod error;
mod queries;

pub use error::{Error, TfaKind, TfaRequired};
use serde::{Deserialize, Serialize};

use crate::models::Me;
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
    /// use bonfire::models::Auth;
    /// use bonfire::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::connect().await.unwrap();
    ///     // ...
    ///     let me = Auth::me(&mut client).await.unwrap();
    ///     println!("logged in as {}", me.name);
    /// }
    /// ```
    pub async fn me(client: &Client) -> Result<Me> {
        Auth::_me(client).await
    }

    pub(crate) async fn login(client: &Client, email: &str, password: &str) -> Result<Self> {
        Auth::_login_email(client, email, password).await
    }

    pub(crate) async fn logout(client: &Client) -> Result<()> {
        Auth::_logout(client).await
    }

    pub(crate) async fn refresh(&self, client: &Client) -> Result<Self> {
        Auth::_login_refresh(self, client).await
    }
}
