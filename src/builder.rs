use crate::connectors::Connector;
use crate::request_kind::RequestKind;
use crate::result::Result;
use crate::Session;

/// A session builder. A `Builder` can be used to construct a `Session` through a builder-like pattern.
pub struct Builder {
    pub authorization: String,
    pub kind: RequestKind,
}
impl Builder {
    /// Create a new `Builder` with default arguments.
    ///
    /// ```
    /// # use bonfire::session::*;
    /// #
    /// let mut builder: &mut Builder = &mut Builder::new();
    /// ```
    pub fn new() -> Self {
        Self {
            authorization: String::new(),
            kind: RequestKind::default(),
        }
    }

    /// Connect to the server.
    ///
    /// ```no_run
    /// # use std::net::{SocketAddr, IpAddr, Ipv4Addr};
    /// # use bonfire::session::*;
    /// #
    /// # let host = "localhost";
    /// # let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    /// # tokio_test::block_on(async {
    /// #
    /// let mut session = Builder::new()
    ///     /* ... */
    ///     .connect(SecureConnector::new(host, addr)).await?;
    /// #
    /// # Ok::<(), Error>(()) }).unwrap();
    /// ```
    pub async fn connect<C>(&self, connector: C) -> Result<Session>
    where
        C: Connector,
    {
        Session::connect(self, connector).await
    }

    /// Set the authorization header for this `Session`.
    ///
    /// ```
    /// # use bonfire::session::*;
    /// #
    /// # let mut builder: &mut Builder = &mut Builder::new();
    /// builder = builder.authorization("Bearer <token>");
    /// ```
    pub fn authorization<T>(&mut self, authorization: T) -> &mut Self
    where
        String: From<T>,
    {
        self.authorization = authorization.into();
        self
    }

    /// Set the type of this `Session`.
    ///
    /// ```
    /// # use bonfire::session::*;
    /// #
    /// # let mut builder: &mut Builder = &mut Builder::new();
    /// builder = builder.kind(RequestKind::default());
    /// ```
    pub fn kind(&mut self, kind: RequestKind) -> &mut Self {
        self.kind = kind;
        self
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}
