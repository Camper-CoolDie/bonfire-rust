mod error;
mod requests;

pub use error::Error;
use requests::{
    AndroidCheckinRequest, AndroidRegisterRequest, InstallationRequest, RegisterRequest,
    UnregisterRequest,
};

use crate::Result;
use crate::client::HyperClient;
use crate::models::{FcmAndroidRegistration, FcmCredentials, FirebaseConfig};

#[derive(Debug)]
pub(crate) struct FcmService {
    config: FirebaseConfig,
}

impl FcmService {
    pub(crate) fn new(config: FirebaseConfig) -> Self {
        Self { config }
    }

    pub(crate) async fn register_android(
        &self,
        client: &HyperClient,
    ) -> Result<FcmAndroidRegistration> {
        let (installation_auth_token, (android_id, security_token, gcm_token)) = tokio::try_join!(
            async {
                tracing::debug!("sending installation request");
                InstallationRequest::new(
                    &self.config.project_id,
                    &self.config.app_id,
                    &self.config.api_key,
                )
                .send_request(client)
                .await
                .inspect_err(|error| tracing::error!(?error, "failed to send installation request"))
            },
            async {
                tracing::debug!("sending Android checkin request");
                let (android_id, security_token) = AndroidCheckinRequest::new()
                    .send_request(client)
                    .await
                    .inspect_err(|error| {
                        tracing::error!(?error, "failed to send Android checkin request");
                    })?
                    .try_into()?;

                tracing::debug!("sending Android registration request");
                let gcm_token =
                    AndroidRegisterRequest::new(&self.config.app_id, android_id, security_token)
                        .send_request(client)
                        .await
                        .inspect_err(|error| {
                            tracing::error!(?error, "failed to send Android registration request");
                        })?;

                Ok((android_id, security_token, gcm_token))
            }
        )?;

        Ok(FcmAndroidRegistration {
            installation_auth_token,
            android_id,
            security_token,
            gcm_token,
        })
    }

    pub(crate) async fn register(
        &self,
        client: &HyperClient,
        android: FcmAndroidRegistration,
    ) -> Result<FcmCredentials> {
        let response = RegisterRequest::new(
            &self.config.project_id,
            &self.config.api_key,
            self.config.vapid_key.as_deref(),
            &android.installation_auth_token,
            &android.gcm_token,
        )
        .send_request(client)
        .await
        .inspect_err(|error| {
            tracing::error!(?error, "failed to send registration request");
        })?;

        Ok(FcmCredentials {
            token: response.token,
            public_key: response.public_key,
            private_key: response.private_key,
            auth_secret: response.auth_secret,
            android,
        })
    }

    pub(crate) async fn unregister(
        &self,
        client: &HyperClient,
        android: &FcmAndroidRegistration,
        fcm_token: &str,
    ) -> Result<()> {
        UnregisterRequest::new(
            &self.config.project_id,
            &self.config.api_key,
            &android.installation_auth_token,
            fcm_token,
        )
        .send_request(client)
        .await
        .inspect_err(|error| {
            tracing::error!(?error, "failed to send unregistration request");
        })?;

        Ok(())
    }
}
