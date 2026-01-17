use tokio::sync::RwLock;

use crate::client::is_token_expired;
use crate::models::Auth;
use crate::{Client, Result};

pub(super) struct TokenProvider {
    auth: RwLock<Option<Auth>>,
}
impl TokenProvider {
    pub(super) fn new(auth: Option<Auth>) -> Self {
        TokenProvider {
            auth: RwLock::new(auth),
        }
    }

    pub(super) async fn get_auth(&self, client: &Client) -> Result<Option<Auth>> {
        self.check_and_refresh(client).await?;
        Ok(self.auth.read().await.clone())
    }

    pub(super) async fn get_token(&self, client: &Client) -> Result<Option<String>> {
        Ok(self.get_auth(client).await?.map(|auth| auth.access_token))
    }

    pub(super) async fn is_auth(&self) -> bool {
        self.auth.read().await.is_some()
    }

    pub(super) async fn set_auth(&self, auth: Option<Auth>) {
        let mut guard = self.auth.write().await;
        *guard = auth;
    }

    async fn check_and_refresh(&self, client: &Client) -> Result<()> {
        let guard = self.auth.read().await;
        match guard.as_ref() {
            Some(auth) if is_token_expired(auth)? => {
                tracing::debug!("Login has expired, refreshing");
                drop(guard);
                self.refresh_now(client).await
            }
            _ => Ok(()),
        }
    }

    async fn refresh_now(&self, client: &Client) -> Result<()> {
        let mut guard = self.auth.write().await;
        *guard = match guard.as_ref() {
            // Another thread may already have refreshed auth, so we check if it's still expired
            Some(auth) if is_token_expired(auth)? => Some(auth.refresh(client).await?),
            _ => None,
        };
        Ok(())
    }
}
