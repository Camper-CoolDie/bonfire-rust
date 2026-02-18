use std::fmt;
use std::sync::LazyLock;

use http::Uri;

use crate::client::jwt::{decode_token, JwtResult};
use crate::models::Auth;
use crate::Client;

// It's great when we can test our requests against a test server, hence the ability to specify
// custom URIs
static ROOT_SERVER_URI: LazyLock<Uri> =
    LazyLock::new(|| Uri::from_static("https://cf2.bonfire.moe"));
static MELIOR_SERVER_URI: LazyLock<Uri> =
    LazyLock::new(|| Uri::from_static("https://api.bonfire.moe"));

/// A builder-like pattern for constructing and configuring a [`Client`] instance.
pub struct ClientBuilder {
    root_uri: Uri,
    melior_uri: Uri,
    auth: Option<Auth>,
}
impl ClientBuilder {
    /// Creates a new `ClientBuilder` with default API endpoint URIs and no authentication
    /// credentials.
    #[must_use]
    pub fn new() -> Self {
        Self {
            root_uri: ROOT_SERVER_URI.clone(),
            melior_uri: MELIOR_SERVER_URI.clone(),
            auth: None,
        }
    }

    /// Consumes the `ClientBuilder` and creates a [`Client`] instance.
    pub fn build(self) -> Client {
        Client::new(&self.root_uri, &self.melior_uri, self.auth)
    }

    /// Sets the URI for the Root API server.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument cannot be converted to a valid `Uri`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bonfire::ClientBuilder;
    /// #
    /// let client = &ClientBuilder::new()
    ///     .root_uri("http://localhost:7070")
    ///     .build();
    /// ```
    #[must_use]
    pub fn root_uri<T>(mut self, uri: T) -> Self
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: fmt::Debug,
    {
        self.root_uri = Uri::try_from(uri).unwrap();
        self
    }

    /// Sets the URI for the Melior API server.
    ///
    /// # Panics
    ///
    /// Panics if the provided argument cannot be converted to a valid `Uri`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bonfire::ClientBuilder;
    /// #
    /// let client = &ClientBuilder::new()
    ///     .melior_uri("http://localhost:8000")
    ///     .build();
    /// ```
    #[must_use]
    pub fn melior_uri<T>(mut self, uri: T) -> Self
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: fmt::Debug,
    {
        self.melior_uri = Uri::try_from(uri).unwrap();
        self
    }

    /// Sets the initial authentication credentials for the client.
    ///
    /// # Errors
    ///
    /// Returns [`JwtError`][crate::client::JwtError] if an error occurs while parsing the provided
    /// credentials.
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
    /// let client = &ClientBuilder::new()
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
