mod heartbeat;
mod parse;

use std::collections::VecDeque;
use std::error::Error as StdError;
use std::panic;
use std::time::{Duration, Instant};

use backon::{BackoffBuilder as _, FibonacciBuilder, Retryable as _};
use ece::EcKeyComponents;
use futures::{FutureExt as _, Stream};
use heartbeat::{Command as HeartbeatCommand, Heartbeat};
pub use parse::Parse;
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio::task::JoinSet;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::CancellationToken;
use tracing::instrument;

use crate::models::{Message, RawMessage, Subscription};
use crate::requests::LoginRequest;
use crate::{Connection, Error, Result};

// Minimum duration of `task_loop()` before it encountered an error and can be retried (5 mins)
const MIN_DURATION_FOR_RETRY: Duration = Duration::from_secs(5 * 60);

#[derive(Deserialize)]
struct MessageData {
    #[serde(rename = "data")]
    content: serde_json::Value,
}

pub(super) struct Listener;
impl Listener {
    pub(super) fn spawn<P: Parse>(
        parser: P,
        subscription: Subscription,
        android_id: u64,
        security_token: u64,
        cancellation_token: CancellationToken,
        buffer: usize,
    ) -> impl Stream<Item = P::Target> {
        let (sender, receiver) = mpsc::channel(buffer);
        tokio::spawn(Self::task(
            parser,
            subscription,
            sender,
            cancellation_token,
            android_id,
            security_token,
        ));
        ReceiverStream::new(receiver)
    }

    #[instrument(name = "listener", skip_all, fields(id = subscription.id))]
    async fn task<P: Parse>(
        parser: P,
        mut subscription: Subscription,
        sender: mpsc::Sender<P::Target>,
        cancellation_token: CancellationToken,
        android_id: u64,
        security_token: u64,
    ) {
        loop {
            let started_at = Instant::now();
            let (error, should_retry) = Self::task_loop(
                &parser,
                &mut subscription,
                &sender,
                &cancellation_token,
                android_id,
                security_token,
            )
            .await
            .map_or((None, false), |(error, should_retry)| {
                (Some(error), should_retry)
            });

            // If `task_loop()` has been running for a long enough time, try reconnecting
            if should_retry && started_at.elapsed() > MIN_DURATION_FOR_RETRY {
                tracing::info!(?error, elapsed = ?started_at.elapsed(), "reconnecting");
                continue;
            }

            // Pass subscription with modified `persistent_ids` to the parser
            parser.stop(subscription, error).await;
            break;
        }
    }

    async fn task_loop<P: Parse>(
        parser: &P,
        subscription: &mut Subscription,
        sender: &mpsc::Sender<P::Target>,
        cancellation_token: &CancellationToken,
        android_id: u64,
        security_token: u64,
    ) -> Option<(P::Error, bool)> {
        let id = subscription.id;
        let key_components = &subscription.key_components;
        let auth_secret = &subscription.auth_secret;
        let persistent_ids = &mut subscription.persistent_ids;
        let connection = match Self::connect_and_login(
            key_components,
            auth_secret,
            persistent_ids,
            android_id,
            security_token,
        )
        .await
        {
            Ok(connection) => connection,
            Err(error) => return Some(Self::map_error(error)),
        };

        // JoinSet aborts the underlying task when dropped
        let mut join_set = JoinSet::new();
        let (heartbeat_sender, heartbeat_receiver) = mpsc::channel(1);
        join_set.spawn(Heartbeat::task(
            heartbeat_receiver,
            connection.clone(),
            cancellation_token.clone(),
            id,
        ));
        let heartbeat_future = join_set.join_next().map(|option| {
            match option {
                Some(Ok(result)) => result,
                Some(Err(error)) => {
                    match error.try_into_panic() {
                        Ok(reason) => panic::resume_unwind(reason),
                        // The task was aborted
                        Err(_) => Ok(()),
                    }
                }
                None => Ok(()),
            }
        });
        tokio::pin!(heartbeat_future);

        loop {
            tokio::select! {
                // Heartbeat task must finish first before stopping the listener
                result = &mut heartbeat_future => {
                    match result {
                        Ok(()) => {
                            tracing::debug!("stopping");
                            break;
                        }
                        Err(error) => return Some(Self::map_error(error)),
                    }
                }
                result = connection.read() => {
                    let (option, should_close) = match result {
                        Ok(Some(message)) => {
                            match Self::parse_message(message, persistent_ids, &heartbeat_sender)
                                .await
                            {
                                Ok((option, should_close)) => (option, should_close),
                                Err(error) => {
                                    return Some(Self::map_error(error));
                                }
                            }
                        }
                        Ok(None) => continue,
                        Err(error) => {
                            tracing::error!(?error, "failed to read next message");
                            return Some(Self::map_error(error));
                        }
                    };

                    if let Some(message) = option {
                        match parser.parse(message).await {
                            Ok(Some(target)) => {
                                if sender.send(target).await.is_err() {
                                    tracing::warn!("receiver has been dropped abruptly");
                                    break;
                                }
                            }
                            Ok(None) => {}
                            Err(error) => {
                                tracing::error!(?error, "failed to parse message");
                                return Some((error, false));
                            }
                        }
                    }

                    if should_close {
                        break;
                    }
                }
            }
        }

        connection.shutdown().await.err().map(Self::map_error)
    }

    fn map_error<E: From<Error> + StdError + Send>(error: Error) -> (E, bool) {
        let should_retry = Self::is_retryable(&error);
        (error.into(), should_retry)
    }

    fn is_retryable(error: &Error) -> bool {
        matches!(error, Error::IoError(_))
    }

    async fn connect_and_login(
        key_components: &EcKeyComponents,
        auth_secret: &[u8; 16],
        persistent_ids: &VecDeque<String>,
        android_id: u64,
        security_token: u64,
    ) -> Result<Connection> {
        // Retry durations: 5s, 5s, 10s, 15s, 25s
        let backoff = FibonacciBuilder::new()
            .with_min_delay(Duration::from_secs(5))
            .with_max_times(5)
            .build();

        (async || {
            tracing::info!("connecting to MSC");
            let connection = Connection::connect(key_components.clone(), *auth_secret)
                .await
                .inspect_err(|error| tracing::error!(?error, "failed to connect to MSC"))?;

            tracing::info!("logging in");
            LoginRequest::new(android_id, security_token, persistent_ids)
                .send_message(&connection)
                .await
                .inspect_err(|error| {
                    tracing::error!(?error, "failed to log in");
                })?;

            Ok(connection)
        })
        .retry(backoff)
        .when(Self::is_retryable)
        .notify(|_, duration| tracing::info!("reconnecting in {duration:?}"))
        .await
    }

    async fn parse_message(
        message: RawMessage,
        persistent_ids: &mut VecDeque<String>,
        heartbeat_sender: &mpsc::Sender<HeartbeatCommand>,
    ) -> Result<(Option<Message>, bool)> {
        match message {
            RawMessage::Data {
                persistent_id,
                body,
            } => {
                Self::push_persistent_id(persistent_ids, persistent_id);
                let content = serde_json::from_slice::<MessageData>(&body)?.content;
                let _ = heartbeat_sender
                    .send(HeartbeatCommand::MessageReceived)
                    .await;
                Ok((Some(Message::Data(content)), false))
            }
            RawMessage::MessagesDeleted {
                persistent_id,
                count,
            } => {
                Self::push_persistent_id(persistent_ids, persistent_id);
                let _ = heartbeat_sender
                    .send(HeartbeatCommand::MessageReceived)
                    .await;
                Ok((Some(Message::MessagesDeleted { count }), false))
            }
            RawMessage::HeartbeatPing(ping) => {
                let _ = heartbeat_sender
                    .send(HeartbeatCommand::Pinged {
                        status: ping.status,
                    })
                    .await;
                Ok((None, false))
            }
            RawMessage::HeartbeatAck(_) => {
                let _ = heartbeat_sender.send(HeartbeatCommand::Acked).await;
                Ok((None, false))
            }
            RawMessage::Close => {
                tracing::info!("closing connection");
                Ok((None, true))
            }
            other => Err(Error::McsProtocolError(format!(
                "unexpected tag: {:?}",
                other.kind()
            ))),
        }
    }

    fn push_persistent_id(ids: &mut VecDeque<String>, id: String) {
        if ids.len() == Subscription::PERSISTENT_IDS_MAX_COUNT {
            ids.pop_front();
        }
        ids.push_back(id);
    }
}
