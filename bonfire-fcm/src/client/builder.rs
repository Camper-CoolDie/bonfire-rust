use crate::Client;
use crate::models::{Config, Credentials};

pub struct Builder {
    config: Config,
    credentials: Option<Credentials>,
}
impl Builder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            credentials: None,
        }
    }

    #[must_use]
    pub fn build(self) -> Client {
        Client::new(self.config, self.credentials)
    }

    #[must_use]
    pub fn project_id<T>(mut self, id: T) -> Self
    where
        String: From<T>,
    {
        self.config.project_id = id.into();
        self
    }

    #[must_use]
    pub fn app_id<T>(mut self, id: T) -> Self
    where
        String: From<T>,
    {
        self.config.app_id = id.into();
        self
    }

    #[must_use]
    pub fn api_key<T>(mut self, key: T) -> Self
    where
        String: From<T>,
    {
        self.config.api_key = key.into();
        self
    }

    #[must_use]
    pub fn vapid_key<T>(mut self, key: T) -> Self
    where
        String: From<T>,
    {
        self.config.vapid_key = key.into();
        self
    }

    #[must_use]
    pub fn uri<T>(mut self, uri: T) -> Self
    where
        String: From<T>,
    {
        self.config.uri = uri.into();
        self
    }

    pub fn credentials(mut self, credentials: Credentials) -> jwt::Result<Self> {
        jwt::decode(&credentials.installation_auth_token)?;
        self.credentials = Some(credentials);
        Ok(self)
    }
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}
