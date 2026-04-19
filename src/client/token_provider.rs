use std::sync::Arc;

use chrono::Utc;
use tokio::sync::RwLock;

use crate::client::jwt::{JwtClaims, JwtResult, decode_token};
use crate::client::{JwtError, Request};
use crate::models::Auth;
use crate::queries::auth::RefreshQuery;
use crate::{Client, Result};

#[derive(Debug)]
enum InnerState {
    Authenticated(Auth, JwtClaims),
    Unauthenticated,
    // A "poisoned" state, for cases when the server sends an invalid token
    InvalidToken(Arc<JwtError>),
}
impl InnerState {
    fn is_auth(&self) -> bool {
        matches!(self, InnerState::Authenticated(_, _))
    }
}

impl TryFrom<Option<Auth>> for InnerState {
    type Error = JwtError;

    fn try_from(value: Option<Auth>) -> JwtResult<Self> {
        match value {
            Some(auth) => {
                let claims = decode_token(&auth.access_token)?;
                Ok(InnerState::Authenticated(auth, claims))
            }
            None => Ok(InnerState::Unauthenticated),
        }
    }
}

#[derive(Debug)]
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

    pub(super) async fn auth(&self, client: &Client) -> Result<Option<Auth>> {
        let guard = self.inner.read().await;
        match &*guard {
            InnerState::Authenticated(_, claims) if claims.expires_at < Utc::now() => {
                drop(guard);
                self.check_and_refresh(client).await
            }
            InnerState::Authenticated(auth, _) => Ok(Some(auth.clone())),
            InnerState::Unauthenticated => Ok(None),
            InnerState::InvalidToken(error) => Err(Arc::clone(error).into()),
        }
    }

    pub(super) async fn token(&self, client: &Client) -> Result<Option<String>> {
        Ok(self.auth(client).await?.map(|auth| auth.access_token))
    }

    pub(super) async fn set_auth(&self, auth: Option<Auth>) -> JwtResult<()> {
        let mut guard = self.inner.write().await;
        *guard = InnerState::try_from(auth)?;
        Ok(())
    }

    async fn check_and_refresh(&self, client: &Client) -> Result<Option<Auth>> {
        let mut guard = self.inner.write().await;

        // If `refresh_now()` fails, the error is returned immediately without poisoning, so the
        // same request can be sent again later
        let auth = match &*guard {
            InnerState::Authenticated(auth, claims) if claims.expires_at < Utc::now() => {
                tracing::debug!(expires_at = ?claims.expires_at, "auth has expired, refreshing");
                Self::refresh_now(client, auth).await?
            }
            InnerState::Authenticated(auth, _) => return Ok(Some(auth.clone())),
            InnerState::Unauthenticated => return Ok(None),
            InnerState::InvalidToken(error) => return Err(Arc::clone(error).into()),
        };

        // If `try_into()` fails, the server has sent an invalid token, so `Auth` enters a poisoned
        // state
        *guard = Some(auth)
            .try_into()
            .unwrap_or_else(|error| InnerState::InvalidToken(Arc::new(error)));
        match &*guard {
            InnerState::Authenticated(auth, _) => Ok(Some(auth.clone())),
            InnerState::Unauthenticated => unreachable!("unauthenticated auth state"),
            InnerState::InvalidToken(error) => Err(Arc::clone(error).into()),
        }
    }

    async fn refresh_now(client: &Client, auth: &Auth) -> Result<Auth> {
        Ok(RefreshQuery::new(&auth.refresh_token)
            .send_request(client)
            .await?
            .into())
    }
}
