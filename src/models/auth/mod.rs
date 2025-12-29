mod error;
mod queries;

pub use error::{Error, TfaKind, TfaRequired};
use serde::{Deserialize, Serialize};

use crate::models::Me;
use crate::{Client, Result};

/// Represents an authentication session.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    /// The session's access token
    pub access_token: String,
    /// The session's refresh token
    pub refresh_token: String,
}
impl Auth {
    /// Log in using email.
    ///
    /// # Errors
    ///
    /// Returns [Error::TfaRequired] if a TFA is required to continue logging in or
    /// [Error][crate::Error] if an error occurred while sending the request.
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
    ///     Auth::login(&mut client, "email", "password").await.unwrap();
    /// }
    /// ```
    pub async fn login(client: &mut Client, email: &str, password: &str) -> Result<()> {
        Auth::_login_email(client, email, password).await
    }

    /// Log out. The client becomes unauthenticated.
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
    ///     Auth::logout(&mut client).await.unwrap();
    /// }
    /// ```
    pub async fn logout(client: &mut Client) -> Result<()> {
        Auth::_logout(client).await
    }

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
    pub async fn me(client: &mut Client) -> Result<Me> {
        Auth::_me(client).await
    }

    pub(crate) async fn refresh(client: &mut Client) -> Result<()> {
        Auth::_login_refresh(client).await
    }
}
