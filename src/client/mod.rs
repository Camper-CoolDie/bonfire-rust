mod error;
mod jwt;
mod session;

use std::fmt;
use std::sync::Arc;

pub use error::{Error, Result};
use http::{header, HeaderMap, Uri};
use serde::de::DeserializeOwned;
use serde::Serialize;
use session::Session;
use tokio::sync::RwLock;

use crate::client::jwt::validate_token;
use crate::models::{auth, Auth, Query, Request};

// It's great when we can test our requests against a test server, hence the ability to specify
// custom URIs
const ROOT_SERVER_URI: &str = "https://cf2.bonfire.moe";
const MELIOR_SERVER_URI: &str = "https://api.bonfire.moe";

// Some requests require this value and return various responses depending on it
const API_VERSION: &str = "3.1.0";

struct Inner {
    root_session: Session,
    melior_session: Session,
    auth: RwLock<Option<Auth>>,
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
                root_session: Session::new(root_uri),
                melior_session: Session::new(melior_uri),
                auth: RwLock::new(auth),
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
        self.inner.auth.read().await
            .as_ref()
            .map_or(Ok(()), |_| Err(auth::Error::AlreadyAuthenticated))?;

        let auth = Auth::login(self, email, password).await?;

        let mut guard = self.inner.auth.write().await;
        *guard = Some(auth);
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
        self.inner.auth.read().await
            .as_ref()
            .map_or(Err(auth::Error::Unauthenticated), |_| Ok(()))?;

        Auth::logout(self).await?;

        let mut guard = self.inner.auth.write().await;
        *guard = None;
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
        // Basically a modified version of `Client::validate_token()`. Why not just call it? We
        // remove the extra `self.inner.auth.read()` which is good for optimization, since the
        // original method returns only the access token and we need the whole credentials.
        let guard = self.inner.auth.read().await;
        match &*guard {
            Some(auth) => {
                if validate_token(auth, false)?.is_some() {
                    return Ok(Some(auth.clone()));
                }
            }
            None => return Ok(None),
        };
        drop(guard);

        let mut guard = self.inner.auth.write().await;
        let auth = match &*guard {
            Some(auth) => {
                if validate_token(auth, false)?.is_some() {
                    return Ok(Some(auth.clone()));
                } else {
                    auth
                }
            }
            None => return Ok(None),
        };

        tracing::info!("login has expired, refreshing");
        let auth = auth.refresh(self).await?;
        validate_token(&auth, true)?;
        *guard = Some(auth.clone());
        Ok(Some(auth))
    }

    async fn validate_token(&self) -> Result<Option<String>> {
        let guard = self.inner.auth.read().await;
        match &*guard {
            Some(auth) => {
                if let Some(token) = validate_token(auth, false)? {
                    return Ok(Some(token.to_owned()));
                }
            }
            None => return Ok(None),
        };
        drop(guard);

        // At this point we need to refresh the access token (if it hasn't already been refreshed
        // by another thread which acquired the write lock first)
        let mut guard = self.inner.auth.write().await;
        let auth = match &*guard {
            Some(auth) => {
                if let Some(token) = validate_token(auth, false)? {
                    return Ok(Some(token));
                } else {
                    auth
                }
            }
            None => return Ok(None),
        };

        // The token is still expired, now it's our job to refresh it
        tracing::info!("login has expired, refreshing");
        let auth = auth.refresh(self).await?;
        let token = validate_token(&auth, true)?;
        // Change `auth` only after validating it
        *guard = Some(auth);
        Ok(token)
    }

    pub(crate) async fn send_request<R: Serialize, S: DeserializeOwned>(
        &self,
        request_name: &'static str,
        content: R,
        attachments: Vec<&[u8]>,
    ) -> Result<S> {
        tracing::info!(request_name, "sending request");
        let token = self.validate_token().await?;

        // Contains the length of each attachment
        let data_output = attachments
            .iter()
            .map(|slice| (!slice.is_empty()).then_some(slice.len() as u32))
            .collect::<Vec<Option<u32>>>();

        let request = Request {
            content,
            request_name,
            data_output,
            api_access_token: token.as_deref(),
            api_version: API_VERSION,
        };

        self.inner
            .root_session
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
        let token = self.validate_token().await?;

        let mut headers = HeaderMap::new();
        if let Some(token) = token {
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap(),
            );
        }

        self.inner
            .melior_session
            .send_query(Query { variables, query }, headers)
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
            .melior_session
            .send_query(Query { variables, query }, HeaderMap::new())
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
    /// let auth = serde_json::from_slice::<Auth>(&auth_data)
    ///     .expect("failed to parse authentication credentials");
    /// let client = ClientBuilder::new().auth(auth).build();
    /// ```
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = Some(auth);
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
