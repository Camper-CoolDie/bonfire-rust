mod error;
mod jwt;
mod query;
mod request;
mod session;

use std::fmt;

pub use error::{Error, Result};
use http::{header, HeaderMap, Uri};
use jsonwebtoken::errors::ErrorKind;
pub use query::MeliorError;
use query::Query;
use request::Request;
pub use request::{RootError, UnavailableError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use session::Session;

use crate::connector;
use crate::models::{auth, Auth};

// It's great when we can test our requests against a test server, hence the ability to specify
// custom URIs.
const ROOT_SERVER_URI: &str = "https://cf2.bonfire.moe";
const MELIOR_SERVER_URI: &str = "https://api.bonfire.moe";

// Some requests require this value and return various responses depending on it.
const API_VERSION: &str = "3.1.0";

/// Represents an HTTP wrapper for the Bonfire API. It manages the authentication session [Auth]
/// and automatically handles token validation and refreshing.
///
/// # Examples
///
/// ```no_run
/// use bonfire::Client;
///
/// #[tokio::main]
/// async fn main() {
///     let mut client = Client::connect().await.unwrap();
///     // ...
/// }
/// ```
pub struct Client {
    /// Client's authentication session (contains tokens)
    pub auth: Option<Auth>,
    root_session: Session,
    melior_session: Session,
    bot_token: Option<String>,
}
impl Client {
    /// Connect to the server using default values.
    ///
    /// # Errors
    ///
    /// Returns [connector::Error] if there was an error while connecting to the servers.
    pub async fn connect() -> connector::Result<Self> {
        Self::builder().connect().await
    }

    /// Validate and refresh an access token if it has expired. This method is automatically called
    /// when a request or query is about to be sent.
    ///
    /// # Errors
    ///
    /// Returns [Error] if there was an error while sending a refresh request, [Error::JwtError] if
    /// an access token can't be validated or
    /// [models::auth::Error::Unauthenticated][crate::models::auth::Error::Unauthenticated] if
    /// the client is unauthenticated.
    pub async fn validate_and_refresh(&mut self) -> Result<()> {
        let auth = self.auth.as_ref().ok_or(auth::Error::Unauthenticated)?;

        match jwt::decode(&auth.access_token) {
            Ok(claims) => {
                tracing::debug!(
                    subject = claims.subject,
                    expires_at = ?claims.expires_at,
                    issued_at = ?claims.issued_at,
                    "validated login"
                );
                Ok(())
            }
            Err(error) => {
                if *error.kind() == ErrorKind::ExpiredSignature {
                    tracing::info!("login has expired, refreshing");
                    Auth::refresh(self).await?;

                    jwt::decode(&self.auth.as_ref().unwrap().access_token).map_err(|error| {
                        tracing::error!(?error, "failed to validate login after refreshing");
                        error
                    })?;
                    Ok(())
                } else {
                    tracing::error!(?error, "failed to validate login");
                    Err(error.into())
                }
            }
        }
    }

    /// Create a new `ClientBuilder` with default values.
    #[inline]
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub(crate) async fn send_request<R: Serialize, S: DeserializeOwned>(
        &mut self,
        request_name: &'static str,
        content: R,
        attachments: Vec<Option<&[u8]>>,
    ) -> Result<S> {
        tracing::info!(request_name, "sending request");

        if self.auth.is_some() {
            self.validate_and_refresh().await?;
        }

        // Contains the length of each attachment
        let data_output = attachments
            .iter()
            .map(|option| option.map(|slice| slice.len() as u32))
            .collect::<Vec<Option<u32>>>();

        let headers = HeaderMap::new();
        let request = Request {
            content,
            request_name,
            data_output,
            api_access_token: match self.auth {
                Some(ref auth) => Some(&auth.access_token),
                None => None,
            },
            api_bot_token: self.bot_token.as_deref(),
            api_version: API_VERSION,
        };

        self.root_session
            .send_request(request, attachments, headers)
            .await
            .map_err(|error| {
                tracing::error!(?error, "failed to send request");
                error
            })
    }

    pub(crate) async fn send_query<R: Serialize, S: DeserializeOwned>(
        &mut self,
        operation_name: &'static str,
        query: &'static str,
        variables: R,
    ) -> Result<S> {
        tracing::info!(operation_name, "sending query");

        let mut headers = HeaderMap::new();
        let query = Query { variables, query };

        if self.auth.is_some() {
            self.validate_and_refresh().await?;

            let auth = self.auth.as_ref().unwrap();
            headers.insert(
                header::AUTHORIZATION,
                format!("Bearer {}", auth.access_token).parse().unwrap(),
            );
        }

        self.melior_session
            .send_query(query, headers)
            .await
            .map_err(|error| {
                tracing::error!(?error, "failed to send query");
                error
            })
    }

    pub(crate) async fn send_query_without_auth<R: Serialize, S: DeserializeOwned>(
        &mut self,
        operation_name: &'static str,
        query: &'static str,
        variables: R,
    ) -> Result<S> {
        tracing::info!(operation_name, "sending query without auth");

        let headers = HeaderMap::new();
        let query = Query { variables, query };

        self.melior_session
            .send_query(query, headers)
            .await
            .map_err(|error| {
                tracing::error!(?error, "failed to send query");
                error
            })
    }
}

/// Represents a builder that can be used to construct a `Client` through a builder-like pattern.
pub struct ClientBuilder {
    root_uri: Uri,
    melior_uri: Uri,
    bot_token: Option<String>,
}
impl ClientBuilder {
    /// Create a new `ClientBuilder` with default values.
    pub fn new() -> Self {
        Self {
            root_uri: Uri::try_from(ROOT_SERVER_URI).unwrap(),
            melior_uri: Uri::try_from(MELIOR_SERVER_URI).unwrap(),
            bot_token: None,
        }
    }

    /// Connect to the server and transform `ClientBuilder` into a `Client`.
    ///
    /// # Errors
    ///
    /// Returns [connector::Error] if there was an error while connecting to the servers.
    pub async fn connect(&self) -> connector::Result<Client> {
        let (root_session, melior_session) = tokio::join!(
            Session::connect(&self.root_uri),
            Session::connect(&self.melior_uri),
        );

        Ok(Client {
            auth: None,
            root_session: root_session?,
            melior_session: melior_session?,
            bot_token: self.bot_token.clone(),
        })
    }

    /// Set the root server's URI.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument couldn't be converted to `Uri`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bonfire::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::builder()
    ///         .root_uri("http://localhost:7070")
    ///         .connect()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn root_uri<T>(&mut self, root_uri: T) -> &mut Self
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: fmt::Debug,
    {
        self.root_uri = Uri::try_from(root_uri).unwrap();
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
    /// ```no_run
    /// use bonfire::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::builder()
    ///         .melior_uri("http://localhost:8000")
    ///         .connect()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn melior_uri<T>(&mut self, melior_uri: T) -> &mut Self
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: fmt::Debug,
    {
        self.melior_uri = Uri::try_from(melior_uri).unwrap();
        self
    }

    /// Set a bot token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bonfire::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::builder()
    ///         .bot_token("my-secret-bot-token")
    ///         .connect()
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub fn bot_token<T>(&mut self, bot_token: T) -> &mut Self
    where
        String: From<T>,
    {
        self.bot_token = Some(String::from(bot_token));
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
