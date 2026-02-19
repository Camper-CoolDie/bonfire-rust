mod builder;
mod error;
mod jwt;
mod request;
mod service;
mod token_provider;

use std::sync::Arc;

pub use builder::ClientBuilder;
pub use error::{Error, Result};
use http::{header, HeaderMap, Uri};
pub use jwt::JwtError;
pub(crate) use request::{
    EmptyResponse, InfallibleRequest, Request, RequestError, RequestErrorSource,
};
use service::{MeliorService, RootService};
use token_provider::TokenProvider;

use crate::models::Auth;
use crate::queries::auth::{LoginEmailQuery, LogoutQuery};
use crate::{MeliorError, MeliorQuery, RootError, RootRequest};

// Some requests require this value and return various responses depending on it
const API_VERSION: &str = "3.1.0";

// The maximum allowed size for any type of attachment (6 MiB)
const ATTACHMENT_MAX_SIZE: usize = 6 * 1024 * 1024;

struct Inner {
    root_service: RootService,
    melior_service: MeliorService,
    token_provider: TokenProvider,
}

/// An asynchronous, thread-safe HTTP client for the Bonfire API.
///
/// This client manages the user's authentication session, automatically handling token validation
/// and refreshing. It is designed to be cloned and shared across multiple asynchronous tasks.
///
/// # Examples
///
/// ```no_run
/// use bonfire::models::Account;
/// use bonfire::{Client, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = &Client::default();
///     client.login("user@example.com", "password").await?;
///
///     // Get brief information about the account with id `1`
///     println!("{:#?}", Account::get_by_id(client, 1).await?);
///
///     Ok(())
/// }
/// ```
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}
impl Client {
    fn new(root_uri: &Uri, melior_uri: &Uri, auth: Option<Auth>) -> Self {
        Self {
            inner: Arc::new(Inner {
                root_service: RootService::new(root_uri),
                melior_service: MeliorService::new(melior_uri),
                // This error was previously caught in ClientBuilder::auth()
                token_provider: TokenProvider::new(auth).expect("failed to create TokenProvider"),
            }),
        }
    }

    /// Checks if the client is currently authenticated.
    ///
    /// This method does not attempt to refresh tokens; it only reflects the current authentication
    /// state.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// client.login("email", "password").await?;
    /// assert!(client.is_auth().await); // is_auth() is true
    ///
    /// client.logout().await?;
    /// assert!(!client.is_auth().await); // is_auth() is false
    /// #     Ok(())
    /// # }
    /// ```
    #[must_use]
    pub async fn is_auth(&self) -> bool {
        self.inner.token_provider.is_auth().await
    }

    /// Logs the client into Bonfire using email and password.
    ///
    /// This method can also be used to establish a new, valid session if the client is in an
    /// invalid token state.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::AlreadyAuthenticated`] if the client is already authenticated. Call
    ///   [`Client::logout`][crate::Client::logout] to terminate the current session before logging
    ///   in again.
    /// * Returns [`LoginError::InvalidEmail`][crate::models::auth::LoginError::InvalidEmail],
    ///   [`LoginError::WrongEmail`][crate::models::auth::LoginError::WrongEmail], or
    ///   [`LoginError::WrongPassword`][crate::models::auth::LoginError::WrongPassword] if the
    ///   provided credentials are incorrect.
    /// * Returns [`LoginError::HardBanned`][crate::models::auth::LoginError::HardBanned] if the
    ///   account is permanently banned.
    /// * Returns [`LoginError::TfaRequired`][crate::models::auth::LoginError::TfaRequired] if
    ///   Two-Factor Authentication (TFA) is required to complete the login process.
    /// * Returns [`Error`] if any other error occurs while sending the login request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = &Client::default();
    /// client.login("email", "password").await?;
    /// // You can now send requests using `client` as an authenticated user
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn login(&self, email: &str, password: &str) -> Result<&Self> {
        if self.inner.token_provider.is_auth().await {
            Err(Error::AlreadyAuthenticated)?;
        }

        let auth = Auth::try_from(
            LoginEmailQuery::new(email, password)
                .send_request(self)
                .await?,
        )?;
        self.inner.token_provider.set_auth(Some(auth)).await?;
        Ok(self)
    }

    /// Logs the client out, invalidating the current authentication session.
    ///
    /// After logging out, the client becomes unauthenticated but can still send requests that do
    /// not require authentication.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::Unauthenticated`] if the client is already unauthenticated.
    /// * Returns [`LogoutError::HardBanned`][crate::models::auth::LogoutError::HardBanned] if the
    ///   account attempting to log out is permanently banned.
    /// * Returns [`Error`] if any other error occurs while sending the logout request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// client.login("email", "password").await?;
    /// // ...
    /// client.logout().await?;
    /// // `client` is no longer authenticated, but can still send some requests which don't
    /// // require authentication
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn logout(&self) -> Result<&Self> {
        if !self.inner.token_provider.is_auth().await {
            Err(Error::Unauthenticated)?;
        }

        LogoutQuery::new().send_request(self).await?;
        self.inner.token_provider.set_auth(None).await?;
        Ok(self)
    }

    /// Retrieves the current authentication credentials.
    ///
    /// This method ensures the returned credentials are valid by automatically refreshing them if
    /// they are expired.
    ///
    /// This method is typically called at the end of a program's execution to save the valid
    /// credentials securely for use in [`ClientBuilder::auth()`] when the program restarts.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::Unauthenticated`] if the client is not authenticated.
    /// * Returns [`RefreshError::TokenExpired`][crate::models::auth::RefreshError::TokenExpired] if
    ///   the refresh token has expired, requiring a new login.
    /// * Returns [`Error::JwtError`] if the session becomes invalid after a token refresh (e.g.,
    ///   the server returns a malformed token).
    /// * Returns [`Error`] if any other error occurs while sending the refresh request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::Client;
    /// use std::fs::File;
    /// use std::io::Write;
    ///
    /// use anyhow::Result;
    ///
    /// async fn save_credentials(client: &Client) -> Result<()> {
    ///     let data = serde_json::to_string(&client.auth().await?)?;
    ///     let mut file = File::create("credentials.json")?;
    ///     file.write_all(data.as_bytes())?;
    ///     Ok(())
    /// }
    ///
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = &Client::default();
    /// client.login("email", "password").await?;
    /// // ...
    /// save_credentials(client).await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn auth(&self) -> Result<Auth> {
        self.inner
            .token_provider
            .get_auth(self)
            .await?
            .ok_or(Error::Unauthenticated)
    }

    pub(crate) async fn send_request<R: Request>(
        &self,
        request_name: &'static str,
        content: &R,
        attachments: Vec<&[u8]>,
    ) -> Result<R::Response>
    where
        for<'a> &'a <R::Error as RequestError>::Source: From<&'a RootError>,
    {
        let token = self.inner.token_provider.get_token(self).await?;
        tracing::info!(request_name, "sending request");

        // Contains the length of each attachment
        let data_output = attachments
            .iter()
            .map(|slice| {
                (!slice.is_empty())
                    .then(|| {
                        let length = slice.len();

                        #[allow(clippy::cast_possible_truncation)]
                        #[allow(clippy::cast_possible_wrap)]
                        if length > ATTACHMENT_MAX_SIZE {
                            Err(Error::AttachmentTooLarge)
                        } else {
                            Ok(length as i32)
                        }
                    })
                    .transpose()
            })
            .collect::<Result<Vec<Option<i32>>>>()?;

        let request = RootRequest {
            content,
            request_name,
            data_output,
            api_access_token: token.as_deref(),
            api_version: API_VERSION,
        };

        self.inner
            .root_service
            .send_request(request, attachments, HeaderMap::new())
            .await
            .inspect_err(|error| tracing::error!(?error, "failed to send request"))
    }

    pub(crate) async fn send_query<R: Request>(
        &self,
        operation_name: &'static str,
        query: &'static str,
        variables: &R,
    ) -> Result<R::Response>
    where
        for<'a> &'a <R::Error as RequestError>::Source: From<&'a MeliorError>,
    {
        let token = self.inner.token_provider.get_token(self).await?;
        tracing::info!(operation_name, "sending query");

        let mut headers = HeaderMap::new();
        if let Some(token) = token {
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {token}").parse().unwrap(),
            );
        }

        self.inner
            .melior_service
            .send_query(MeliorQuery { variables, query }, headers)
            .await
            .inspect_err(|error| tracing::error!(?error, "failed to send query"))
    }

    pub(crate) async fn send_query_authless<R: Request>(
        &self,
        operation_name: &'static str,
        query: &'static str,
        variables: &R,
    ) -> Result<R::Response>
    where
        for<'a> &'a <R::Error as RequestError>::Source: From<&'a MeliorError>,
    {
        tracing::info!(operation_name, "sending query without auth");

        self.inner
            .melior_service
            .send_query(MeliorQuery { variables, query }, HeaderMap::new())
            .await
            .inspect_err(|error| tracing::error!(?error, "failed to send an authless query"))
    }

    /// Create a new `ClientBuilder` with default values.
    #[must_use]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}
