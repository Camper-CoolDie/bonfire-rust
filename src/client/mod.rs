mod error;
mod jwt;
mod request;
mod service;
mod token_provider;

use std::fmt;
use std::sync::Arc;

pub use error::{Error, Result};
use http::{header, HeaderMap, Uri};
pub use jwt::JwtError;
use jwt::JwtResult;
pub(crate) use request::{EmptyResponse, Request};
use serde::de::DeserializeOwned;
use serde::Serialize;
use service::{MeliorService, RootService};
use token_provider::TokenProvider;

use crate::client::jwt::decode_token;
use crate::models::{auth, Auth};
use crate::{MeliorQuery, RootRequest};

// It's great when we can test our requests against a test server, hence the ability to specify
// custom URIs
const ROOT_SERVER_URI: &str = "https://cf2.bonfire.moe";
const MELIOR_SERVER_URI: &str = "https://api.bonfire.moe";

// Some requests require this value and return various responses depending on it
const API_VERSION: &str = "3.1.0";

struct Inner {
    root_service: RootService,
    melior_service: MeliorService,
    token_provider: TokenProvider,
}

/// Represents an HTTP wrapper for the Bonfire API. It manages the authentication session [Auth]
/// and automatically handles token validation and refreshing.
///
/// # Examples
///
/// ```no_run
/// use bonfire::models::Account;
/// use bonfire::{Client, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = Client::default();
///     client.login("user@example.com", "password").await?;
///
///     // Get brief information about the account with id `1`
///     println!("{:#?}", Account::get_by_id(&client, 1).await?);
///
///     Ok(())
/// }
/// ```
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

    /// Log in using email.
    ///
    /// # Errors
    ///
    /// * [auth::Error::AlreadyAuthenticated] if the client is already authenticated. You should
    ///   call [Client::logout()] to terminate the current session
    /// * [auth::Error::TfaRequired] if TFA is required to continue logging in
    /// * [Error] if any other error occurred while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = Client::default();
    /// client.login("email", "password").await?;
    /// // You can now send requests using `client` as an authenticated user
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn login(&self, email: &str, password: &str) -> Result<()> {
        if self.inner.token_provider.is_auth().await {
            Err(auth::Error::AlreadyAuthenticated)?;
        }

        let auth = Auth::login(self, email, password).await?;
        self.inner.token_provider.set_auth(Some(auth)).await?;
        Ok(())
    }

    /// Log out. The client becomes unauthenticated.
    ///
    /// # Errors
    ///
    /// Returns [auth::Error::Unauthenticated] if the client is unauthenticated or [Error] if any
    /// other error occurred while sending the log out request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = Client::default();
    /// client.login("email", "password").await?;
    /// // ...
    /// client.logout().await?;
    /// // `client` is no longer authenticated, but can still send some requests which don't
    /// // require authentication
    /// #     Ok(())
    /// # }
    /// ```
    pub async fn logout(&self) -> Result<()> {
        if !self.inner.token_provider.is_auth().await {
            Err(auth::Error::Unauthenticated)?;
        }

        Auth::logout(self).await?;
        self.inner.token_provider.set_auth(None).await?;
        Ok(())
    }

    /// Get the current authentication credentials [Auth]. Are guaranteed to be valid after calling
    /// this method since they refresh if needed. Returns `None` if the client is unauthenticated.
    ///
    /// Is usually called at the end of the program, after which the credentials are saved in a
    /// secure place and used in [ClientBuilder::auth()] when the program runs again.
    ///
    /// # Errors
    ///
    /// Returns [Error] if an error occurred while sending the refresh request.
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
    ///     if let Some(ref auth) = client.auth().await? {
    ///         let data = serde_json::to_string(&auth)?;
    ///         let mut file = File::create("credentials.json")?;
    ///         file.write_all(data.as_bytes())?;
    ///     }
    ///     Ok(())
    /// }
    ///
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = Client::default();
    /// client.login("email", "password").await?;
    /// // ...
    /// save_credentials(&client).await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn auth(&self) -> Result<Option<Auth>> {
        self.inner.token_provider.get_auth(self).await
    }

    pub(crate) async fn send_request<R: Serialize, S: DeserializeOwned>(
        &self,
        request_name: &'static str,
        content: R,
        attachments: Vec<&[u8]>,
    ) -> Result<S> {
        tracing::info!(request_name, "sending request");
        let token = self.inner.token_provider.get_token(self).await?;

        // Contains the length of each attachment
        let data_output = attachments
            .iter()
            .map(|slice| (!slice.is_empty()).then_some(slice.len() as u32))
            .collect::<Vec<Option<u32>>>();

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

    pub(crate) async fn send_query<R: Serialize, S: DeserializeOwned>(
        &self,
        operation_name: &'static str,
        query: &'static str,
        variables: R,
    ) -> Result<S> {
        tracing::info!(operation_name, "sending query");
        let token = self.inner.token_provider.get_token(self).await?;

        let mut headers = HeaderMap::new();
        if let Some(token) = token {
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap(),
            );
        }

        self.inner
            .melior_service
            .send_query(MeliorQuery { variables, query }, headers)
            .await
            .inspect_err(|error| tracing::error!(?error, "failed to send query"))
    }

    // `send_query` that doesn't validate credentials
    pub(crate) async fn send_refresh_query<R: Serialize, S: DeserializeOwned>(
        &self,
        operation_name: &'static str,
        query: &'static str,
        variables: R,
    ) -> Result<S> {
        tracing::info!(operation_name, "sending refresh query");

        self.inner
            .melior_service
            .send_query(MeliorQuery { variables, query }, HeaderMap::new())
            .await
            .inspect_err(|error| tracing::error!(?error, "failed to send refresh query"))
    }

    /// Create a new `ClientBuilder` with default values.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}

/// Represents a builder that can be used to construct a `Client` through a builder-like pattern.
pub struct ClientBuilder {
    root_uri: Uri,
    melior_uri: Uri,
    auth: Option<Auth>,
}
impl ClientBuilder {
    /// Create a new `ClientBuilder` with default values.
    pub fn new() -> Self {
        Self {
            root_uri: Uri::try_from(ROOT_SERVER_URI).unwrap(),
            melior_uri: Uri::try_from(MELIOR_SERVER_URI).unwrap(),
            auth: None,
        }
    }

    /// Builds and transforms `ClientBuilder` into a `Client`.
    pub fn build(self) -> Client {
        Client::new(&self.root_uri, &self.melior_uri, self.auth)
    }

    /// Set the root server's URI.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument couldn't be converted to `Uri`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bonfire::ClientBuilder;
    /// #
    /// let client = ClientBuilder::new()
    ///     .root_uri("http://localhost:7070")
    ///     .build();
    /// ```
    pub fn root_uri<T>(mut self, uri: T) -> Self
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: fmt::Debug,
    {
        self.root_uri = Uri::try_from(uri).unwrap();
        self
    }

    /// Set the melior server's URI.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument couldn't be converted to `Uri`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bonfire::ClientBuilder;
    /// #
    /// let client = ClientBuilder::new()
    ///     .melior_uri("http://localhost:8000")
    ///     .build();
    /// ```
    pub fn melior_uri<T>(mut self, uri: T) -> Self
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: fmt::Debug,
    {
        self.melior_uri = Uri::try_from(uri).unwrap();
        self
    }

    /// Set the authentication credentials.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::ClientBuilder;
    /// use std::fs;
    ///
    /// use bonfire::models::Auth;
    ///
    /// let auth_data = fs::read("credentials.json").expect("failed to read from 'credentials.json'");
    /// let auth = serde_json::from_slice::<Auth>(&auth_data).expect("failed to parse auth");
    /// let client = ClientBuilder::new()
    ///     .auth(auth)
    ///     .expect("invalid auth")
    ///     .build();
    /// ```
    pub fn auth(mut self, auth: Auth) -> JwtResult<Self> {
        decode_token(&auth.access_token)?;
        self.auth = Some(auth);
        Ok(self)
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
