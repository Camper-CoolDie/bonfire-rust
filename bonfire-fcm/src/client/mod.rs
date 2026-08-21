mod builder;
mod connection;
mod error;
mod token_provider;

use std::collections::VecDeque;
use std::sync::Arc;

pub use builder::Builder;
use bytes::Bytes;
pub(super) use connection::Connection;
pub use error::{AndroidRegistrationError, Error, Result};
use futures::Stream;
use http_body_util::{Either, Empty, Full};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use token_provider::TokenProvider;
use tokio_util::sync::CancellationToken;

use crate::Listener;
use crate::models::{Config, Credentials, Message, Subscription};
use crate::requests::{PushRegistrationRequest, PushUnregistrationRequest};

pub(super) type HyperClient = hyper_util::client::legacy::Client<
    HttpsConnector<HttpConnector>,
    Either<Full<Bytes>, Empty<Bytes>>,
>;

#[derive(Debug)]
struct Inner {
    token_provider: TokenProvider,
    config: Config,
}

#[derive(Clone, Debug)]
pub struct Client {
    hyper: HyperClient,
    inner: Arc<Inner>,
}
impl Client {
    fn new(config: Config, credentials: Option<Credentials>) -> Self {
        let connector = HttpsConnectorBuilder::new()
            .with_webpki_roots()
            .https_only()
            .enable_http2()
            .build();

        Self {
            hyper: hyper_util::client::legacy::Client::builder(TokioExecutor::new())
                .build(connector),
            inner: Arc::new(Inner {
                // This error was previously caught in Builder::credentials()
                token_provider: TokenProvider::new(credentials)
                    .expect("failed to create TokenProvider"),
                config,
            }),
        }
    }

    pub async fn credentials(&self) -> Result<Credentials> {
        self.inner
            .token_provider
            .credentials(&self.hyper, &self.inner.config)
            .await
    }

    pub async fn subscribe(&self, id: u64) -> Result<Subscription> {
        let (key_pair, auth_secret) = ece::generate_keypair_and_auth_secret()?;
        let key_components = key_pair.raw_components()?;
        let config = &self.inner.config;
        let credentials = self.credentials().await?;

        tracing::info!("sending subscription request");
        let response = PushRegistrationRequest::new(
            &config.project_id,
            &config.api_key,
            &config.vapid_key,
            &credentials.installation_auth_token,
            &credentials.gcm_token,
            key_components.public_key(),
            &auth_secret,
        )
        .send_request(&self.hyper)
        .await
        .inspect_err(|error| {
            tracing::error!(?error, "failed to subscribe");
        })?;

        Ok(Subscription {
            id,
            push_token: response.token,
            key_components,
            auth_secret,
            persistent_ids: VecDeque::with_capacity(Subscription::PERSISTENT_IDS_MAX_COUNT),
        })
    }

    pub async fn unsubscribe(&self, subscription: Subscription) -> Result<&Self> {
        let config = &self.inner.config;
        let credentials = self.credentials().await?;

        tracing::info!("sending unsubscription request");
        PushUnregistrationRequest::new(
            &config.project_id,
            &config.api_key,
            &credentials.installation_auth_token,
            &subscription.push_token,
        )
        .send_request(&self.hyper)
        .await
        .inspect_err(|error| {
            tracing::error!(?error, "failed to unsubscribe");
        })?;
        Ok(self)
    }

    pub async fn listen(
        &self,
        subscription: Subscription,
        cancellation_token: CancellationToken,
        buffer: usize,
    ) -> Result<impl Stream<Item = Message>> {
        let registration = self
            .inner
            .token_provider
            .android_registration(&self.hyper, &self.inner.config)
            .await?;

        Ok(Listener::spawn(
            subscription,
            registration.android_id,
            registration.security_token,
            cancellation_token,
            buffer,
        ))
    }

    #[must_use]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}
