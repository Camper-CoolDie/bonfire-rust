use chrono::Utc;
use tokio::sync::RwLock;

use crate::client::jwt::{decode_token, JwtClaims};
use crate::client::{JwtError, JwtResult};
use crate::models::Auth;
use crate::{Client, Result};

enum InnerState {
    Authenticated(Auth, JwtClaims),
    Unauthenticated,
}
impl InnerState {
    fn is_auth(&self) -> bool {
        matches!(self, InnerState::Authenticated(_, _))
    }
}

impl TryFrom<Option<Auth>> for InnerState {
    type Error = JwtError;

    fn try_from(value: Option<Auth>) -> JwtResult<Self> {
        value.map_or(Ok(InnerState::Unauthenticated), |auth| {
            let claims = decode_token(&auth.access_token)?;
            Ok(InnerState::Authenticated(auth, claims))
        })
    }
}

pub(super) struct TokenProvider {
    inner: RwLock<InnerState>,
}
impl TokenProvider {
    pub(super) fn new(auth: Option<Auth>) -> JwtResult<Self> {
        Ok(TokenProvider {
            inner: RwLock::new(InnerState::try_from(auth)?),
        })
    }

    pub(super) async fn is_auth(&self) -> bool {
        self.inner.read().await.is_auth()
    }

    pub(super) async fn get_auth(&self, client: &Client) -> Result<Option<Auth>> {
        let guard = self.inner.read().await;
        match &*guard {
            InnerState::Authenticated(_, claims) if claims.expires_at < Utc::now() => {
                drop(guard);
                self.check_and_refresh(client).await
            }
            InnerState::Authenticated(auth, _) => Ok(Some(auth.clone())),
            InnerState::Unauthenticated => Ok(None),
        }
    }

    pub(super) async fn get_token(&self, client: &Client) -> Result<Option<String>> {
        Ok(self.get_auth(client).await?.map(|auth| auth.access_token))
    }

    pub(super) async fn set_auth(&self, auth: Option<Auth>) -> JwtResult<()> {
        let mut guard = self.inner.write().await;
        *guard = InnerState::try_from(auth)?;
        Ok(())
    }

    async fn check_and_refresh(&self, client: &Client) -> Result<Option<Auth>> {
        let mut guard = self.inner.write().await;
        let option = match &*guard {
            InnerState::Authenticated(auth, claims) if claims.expires_at < Utc::now() => {
                tracing::debug!("auth has expired, refreshing");
                Some(auth.refresh(client).await?)
            }
            InnerState::Authenticated(auth, _) => return Ok(Some(auth.clone())),
            InnerState::Unauthenticated => return Ok(None),
        };
        *guard = option.clone().try_into()?;
        Ok(option)
    }
}
