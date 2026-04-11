use std::fmt;
use std::sync::LazyLock;

use governor::Quota;
use http::Uri;
use nonzero_ext::nonzero;

use crate::Client;
use crate::client::jwt::{JwtResult, decode_token};
use crate::models::Auth;

// It's great when we can test our requests against a mock server, hence the ability to specify
// custom URIs
static ROOT_SERVER_URI: LazyLock<Uri> =
    LazyLock::new(|| Uri::from_static("https://cf2.bonfire.moe"));
static MELIOR_SERVER_URI: LazyLock<Uri> =
    LazyLock::new(|| Uri::from_static("https://api.bonfire.moe"));

// 30 requests per minute with a burst of 15 requests
const DEFAULT_QUOTA: Quota = Quota::per_minute(nonzero!(30u32)).allow_burst(nonzero!(15u32));

// Account IDs with AccessLevel::Protoadmin privileges, may change in the future
const PROTOADMIN_IDS: [u64; 1] = [1];

/// A builder-like pattern for constructing and configuring a [`Client`] instance.
pub struct Builder {
    root_uri: Uri,
    melior_uri: Uri,
    auth: Option<Auth>,
    quota: Quota,
    protoadmin_ids: Vec<u64>,
}
impl Builder {
    /// Creates a new `Builder` with default API endpoint URIs and no authentication
    /// credentials.
    #[must_use]
    pub fn new() -> Self {
        Self {
            root_uri: ROOT_SERVER_URI.clone(),
            melior_uri: MELIOR_SERVER_URI.clone(),
            auth: None,
            quota: DEFAULT_QUOTA,
            protoadmin_ids: PROTOADMIN_IDS.into(),
        }
    }

    /// Consumes the `Builder` and creates a [`Client`] instance.
    pub fn build(self) -> Client {
        Client::new(
            self.root_uri,
            self.melior_uri,
            self.auth,
            self.quota,
            self.protoadmin_ids,
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
    /// # #[cfg(feature = "serde")]
    /// # mod wrapper {
    /// #     use bonfire::ClientBuilder;
    /// use std::fs;
    ///
    /// use bonfire::models::Auth;
    ///
    /// #     fn main() {
    /// let auth_data = fs::read("credentials.json").expect("failed to read from 'credentials.json'");
    /// let auth = serde_json::from_slice::<Auth>(&auth_data).expect("failed to parse auth");
    /// let client = &ClientBuilder::new()
    ///     .auth(auth)
    ///     .expect("invalid auth")
    ///     .build();
    /// #     }
    /// # }
    /// ```
    pub fn auth(mut self, auth: Auth) -> JwtResult<Self> {
        decode_token(&auth.access_token)?;
        self.auth = Some(auth);
        Ok(self)
    }

    /// Sets the rate limiting quota for the client.
    ///
    /// The default value is 30 requests per minute, with a burst size of 15 requests. This rate
    /// limit ensures that the application adheres to the server's rate-limiting policy from a
    /// single IP address.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bonfire::ClientBuilder;
    /// use governor::Quota;
    /// use nonzero_ext::nonzero;
    ///
    /// // Set the rate limit to 15 requests per minute with a burst of 5 requests
    /// let client = &ClientBuilder::new()
    ///     .quota(Quota::per_minute(nonzero!(15u32)).allow_burst(nonzero!(5u32)))
    ///     .build();
    /// ```
    #[must_use]
    pub fn quota(mut self, quota: Quota) -> Self {
        self.quota = quota;
        self
    }

    /// Sets a custom list of account IDs that this [`Client`] instance should consider
    /// to have [`AccessLevel::Protoadmin`][crate::models::AccessLevel::Protoadmin] privileges.
    ///
    /// The default list of protoadmin IDs is kept up-to-date with the actual Bonfire server's
    /// understanding of protoadmins.
    ///
    /// # Examples
    ///
    /// ```
    /// # use bonfire::ClientBuilder;
    /// let client = &ClientBuilder::new()
    ///     .protoadmin_ids(vec![1, 1337, 42])
    ///     .build();
    /// ```
    #[must_use]
    pub fn protoadmin_ids(mut self, ids: Vec<u64>) -> Self {
        self.protoadmin_ids = ids;
        self
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
