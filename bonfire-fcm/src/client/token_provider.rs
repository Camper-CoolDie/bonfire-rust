use std::sync::Arc;

use chrono::Utc;
use jwt::Claims;
use tokio::sync::Mutex;

use crate::client::AndroidRegistrationError;
use crate::models::{AndroidRegistration, Config, Credentials, Installation};
use crate::requests::{
    AndroidRegistrationRequest, AndroidRegistrationResponse, CheckInRequest, InstallationRequest,
    RefreshRequest,
};
use crate::{Error, HyperClient, Result};

#[derive(Debug)]
enum InnerState {
    // Claims are decoded from an installation token
    Registered(Credentials, Claims),
    Unregistered,
    // A "poisoned" state, for cases when the server sends an invalid installation token
    InvalidToken(Arc<jwt::Error>),
}

#[derive(Debug)]
pub(super) struct TokenProvider {
    inner: Mutex<InnerState>,
}
impl TokenProvider {
    pub(super) fn new(credentials: Option<Credentials>) -> jwt::Result<Self> {
        Ok(Self {
            inner: Mutex::new(match credentials {
                Some(credentials) => {
                    let claims = jwt::decode(&credentials.installation_auth_token)?;
                    InnerState::Registered(credentials, claims)
                }
                None => InnerState::Unregistered,
            }),
        })
    }

    pub(super) async fn android_registration(
        &self,
        client: &HyperClient,
        config: &Config,
    ) -> Result<AndroidRegistration> {
        let mut guard = self.inner.lock().await;

        if matches!(&*guard, InnerState::Unregistered) {
            *guard = match self.register(client, config).await {
                Ok((credentials, claims)) => InnerState::Registered(credentials, claims),
                Err(Error::JwtError(error)) => InnerState::InvalidToken(error),
                Err(error) => return Err(error),
            };
        }

        match &*guard {
            InnerState::Registered(credentials, _) => Ok(AndroidRegistration {
                android_id: credentials.android_id,
                security_token: credentials.security_token,
                gcm_token: credentials.gcm_token.clone(),
            }),
            InnerState::Unregistered => unreachable!("unregistered credentials state"),
            InnerState::InvalidToken(error) => Err(Arc::clone(error).into()),
        }
    }

    pub(super) async fn credentials(
        &self,
        client: &HyperClient,
        config: &Config,
    ) -> Result<Credentials> {
        let mut guard = self.inner.lock().await;

        // If `register()` or `refresh()` fails, the error is returned immediately without
        // poisoning, so the same request can be sent again later
        if matches!(&*guard, InnerState::Unregistered) {
            *guard = match self.register(client, config).await {
                Ok((credentials, claims)) => InnerState::Registered(credentials, claims),
                Err(Error::JwtError(error)) => InnerState::InvalidToken(error),
                Err(error) => return Err(error),
            };
        } else if let InnerState::Registered(credentials, claims) = &mut *guard
            && claims.expires_at < Utc::now()
        {
            match self.refresh(client, config, credentials, claims).await {
                Ok(()) => {}
                Err(Error::JwtError(error)) => *guard = InnerState::InvalidToken(error),
                Err(error) => return Err(error),
            }
        }

        match &*guard {
            InnerState::Registered(credentials, _) => Ok(credentials.clone()),
            InnerState::Unregistered => unreachable!("unregistered credentials state"),
            InnerState::InvalidToken(error) => Err(Arc::clone(error).into()),
        }
    }

    async fn register(
        &self,
        client: &HyperClient,
        config: &Config,
    ) -> Result<(Credentials, Claims)> {
        let (registration, (installation, claims)) = tokio::try_join!(
            async {
                self.register_android(client, config)
                    .await
                    .inspect_err(|error| {
                        tracing::error!(?error, "failed to register android");
                    })
            },
            async {
                self.install(client, config)
                    .await
                    .inspect_err(|error| tracing::error!(?error, "failed to install"))
            }
        )?;

        Ok((
            Credentials {
                android_id: registration.android_id,
                security_token: registration.security_token,
                gcm_token: registration.gcm_token,
                installation_id: installation.id,
                installation_auth_token: installation.auth_token,
                installation_refresh_token: installation.refresh_token,
            },
            claims,
        ))
    }

    async fn register_android(
        &self,
        client: &HyperClient,
        config: &Config,
    ) -> Result<AndroidRegistration> {
        tracing::info!("sending check-in request");
        let response = CheckInRequest::new().send_request(client).await?;
        let android_id = response
            .android_id
            .ok_or(Error::missing_field("android_id", "check-in response"))?;
        let security_token = response
            .security_token
            .ok_or(Error::missing_field("security_token", "check-in response"))?;

        tracing::info!("sending android registration request");
        let response = AndroidRegistrationRequest::new(
            &config.vapid_key,
            &config.uri,
            android_id,
            security_token,
        )
        .send_request(client)
        .await?;
        let gcm_token = match response {
            AndroidRegistrationResponse::Ok { gcm_token } => gcm_token,
            AndroidRegistrationResponse::Error { code } => {
                return Err(AndroidRegistrationError::new(code).into());
            }
        };

        Ok(AndroidRegistration {
            android_id,
            security_token,
            gcm_token,
        })
    }

    async fn install(
        &self,
        client: &HyperClient,
        config: &Config,
    ) -> Result<(Installation, Claims)> {
        tracing::info!("sending installation request");
        let response =
            InstallationRequest::new(&config.project_id, &config.app_id, &config.api_key)
                .send_request(client)
                .await?;

        // Token expires in 7 days
        let auth_token = response.auth_token.token;
        let claims = jwt::decode(&auth_token)?;

        Ok((
            Installation {
                id: response.id,
                auth_token,
                refresh_token: response.refresh_token,
            },
            claims,
        ))
    }

    async fn refresh(
        &self,
        client: &HyperClient,
        config: &Config,
        credentials: &mut Credentials,
        claims: &mut Claims,
    ) -> Result<()> {
        tracing::debug!(expires_at = ?claims.expires_at, "installation has expired, refreshing");
        let (token, new_claims) = self
            .refresh_installation(
                client,
                config,
                &credentials.installation_id,
                &credentials.installation_refresh_token,
            )
            .await
            .inspect_err(|error| {
                tracing::error!(?error, "failed to refresh installation");
            })?;

        credentials.installation_auth_token = token;
        *claims = new_claims;
        Ok(())
    }

    async fn refresh_installation(
        &self,
        client: &HyperClient,
        config: &Config,
        id: &str,
        refresh_token: &str,
    ) -> Result<(String, Claims)> {
        tracing::info!("sending refresh request");
        let response = RefreshRequest::new(
            &config.project_id,
            &config.app_id,
            &config.api_key,
            id,
            refresh_token,
        )
        .send_request(client)
        .await?;

        let auth_token = response.token;
        let claims = jwt::decode(&auth_token)?;
        Ok((auth_token, claims))
    }
}
