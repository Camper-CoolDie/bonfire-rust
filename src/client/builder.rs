use std::fmt;
use std::sync::LazyLock;

use http::Uri;

use crate::Client;
use crate::client::jwt::{JwtResult, decode_token};
use crate::models::Auth;

// It's great when we can test our requests against a test server, hence the ability to specify
// custom URIs
static ROOT_SERVER_URI: LazyLock<Uri> =
    LazyLock::new(|| Uri::from_static("https://cf2.bonfire.moe"));
static MELIOR_SERVER_URI: LazyLock<Uri> =
    LazyLock::new(|| Uri::from_static("https://api.bonfire.moe"));

const REQUESTS_PER_SECOND: f32 = 0.5;

/// A builder-like pattern for constructing and configuring a [`Client`] instance.
pub struct ClientBuilder {
    root_uri: Uri,
    melior_uri: Uri,
    auth: Option<Auth>,
    requests_per_second: f32,
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
            requests_per_second: REQUESTS_PER_SECOND,
        }
    }

    /// Consumes the `ClientBuilder` and creates a [`Client`] instance.
    pub fn build(self) -> Client {
        Client::new(
            &self.root_uri,
            &self.melior_uri,
            self.auth,
            self.requests_per_second,
        )
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

    /// Sets the maximum number of requests per second for the client. The default value is `0.5` (1
    /// request every 2 seconds).
    ///
    /// The client's request limiting mechanism allows for bursts of requests up to `rate * 30.0`
    /// requests, after which it will block until new tokens are available. This rate limit ensures
    /// that the application adheres to the server's rate-limiting policy from a single IP address.
    ///
    /// # Panics
    ///
    /// Panics if the provided rate is not a positive value or `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bonfire::ClientBuilder;
    /// #
    /// // Set the rate to 1 request every 5 seconds
    /// let client = &ClientBuilder::new().requests_per_second(0.2).build();
    /// ```
    #[must_use]
    pub fn requests_per_second(mut self, rate: f32) -> Self {
        assert!(rate > 0.0, "requests per second must be > 0.0");
        self.requests_per_second = rate;
        self
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
