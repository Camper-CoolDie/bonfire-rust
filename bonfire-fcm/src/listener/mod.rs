mod heartbeat;

use std::collections::VecDeque;
use std::panic;
use std::time::{Duration, Instant};

use backon::{BackoffBuilder as _, FibonacciBuilder, Retryable as _};
use ece::EcKeyComponents;
use futures::{FutureExt as _, Stream};
use heartbeat::{Command as HeartbeatCommand, Heartbeat};
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
    pub(super) fn spawn(
        subscription: Subscription,
        android_id: u64,
        security_token: u64,
        cancellation_token: CancellationToken,
        buffer: usize,
    ) -> impl Stream<Item = Message> {
        let (message_sender, message_receiver) = mpsc::channel(buffer);
        tokio::spawn(Listener::task(
            subscription,
            message_sender,
            cancellation_token,
            android_id,
            security_token,
        ));
        ReceiverStream::new(message_receiver)
    }

    #[instrument(name = "listener", skip_all, fields(id = subscription.id))]
    async fn task(
        mut subscription: Subscription,
        message_sender: mpsc::Sender<Message>,
        cancellation_token: CancellationToken,
        android_id: u64,
        security_token: u64,
    ) {
        loop {
            let started_at = Instant::now();
            let result = Self::task_loop(
                &mut subscription,
                &message_sender,
                &cancellation_token,
                android_id,
                security_token,
            )
            .await;

            // If `task_loop()` has been running for a long enough time, try reconnecting
            if let Err(ref error) = result
                && Self::is_retryable(error)
                && started_at.elapsed() > MIN_DURATION_FOR_RETRY
            {
                tracing::info!(?error, elapsed = ?started_at.elapsed(), "reconnecting");
                continue;
            }

            // Pass subscription with modified `persistent_ids` back to the caller
            let _ = message_sender
                .send(Message::ListenerStopped {
                    subscription,
                    error: result.err(),
                })
                .await
                .inspect_err(|error| {
                    tracing::warn!(
                        ?error,
                        "failed to send ListenerStopped message (receiver closed)"
                    );
                });
            break;
        }
    }

    async fn task_loop(
        subscription: &mut Subscription,
        message_sender: &mpsc::Sender<Message>,
        cancellation_token: &CancellationToken,
        android_id: u64,
        security_token: u64,
    ) -> Result<()> {
        let id = subscription.id;
        let key_components = &subscription.key_components;
        let auth_secret = &subscription.auth_secret;
        let persistent_ids = &mut subscription.persistent_ids;
        let connection = Self::connect_and_login(
            key_components,
            auth_secret,
            persistent_ids,
            android_id,
            security_token,
        )
        .await?;

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
                () = cancellation_token.cancelled() => {
                    tracing::debug!("stopping");
                    // Heartbeat task also finishes on cancellation, so it should be awaited
                    heartbeat_future.await?;
                    break;
                }
                result = &mut heartbeat_future => {
                    match result {
                        Ok(()) => {
                            tracing::debug!("heartbeat task exited, stopping");
                            break;
                        }
                        Err(error) => return Err(error),
                    }
                }
                result = connection.read() => {
                    match result {
                        Ok(Some(message)) => {
                            let should_close = Self::parse_message(
                                message,
                                persistent_ids,
                                message_sender,
                                &heartbeat_sender,
                            )
                            .await?;

                            if should_close {
                                break;
                            }
                        }
                        Ok(None) => {}
                        Err(error) => {
                            tracing::error!(?error, "failed to read next message");
                            return Err(error);
                        }
                    }
                }
            }
        }

        connection.shutdown().await?;
        Ok(())
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

    fn is_retryable(error: &Error) -> bool {
        matches!(error, Error::IoError(_))
    }

    // Returns true if the connection should close
    async fn parse_message(
        message: RawMessage,
        persistent_ids: &mut VecDeque<String>,
        message_sender: &mpsc::Sender<Message>,
        heartbeat_sender: &mpsc::Sender<HeartbeatCommand>,
    ) -> Result<bool> {
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
                Ok(message_sender.send(Message::Data(content)).await.is_err())
            }
            RawMessage::MessagesDeleted {
                persistent_id,
                count,
            } => {
                Self::push_persistent_id(persistent_ids, persistent_id);
                let _ = heartbeat_sender
                    .send(HeartbeatCommand::MessageReceived)
                    .await;
                Ok(message_sender
                    .send(Message::MessagesDeleted { count })
                    .await
                    .is_err())
            }
            RawMessage::HeartbeatPing(ping) => {
                let _ = heartbeat_sender
                    .send(HeartbeatCommand::Pinged {
                        status: ping.status,
                    })
                    .await;
                Ok(false)
            }
            RawMessage::HeartbeatAck(_) => {
                let _ = heartbeat_sender.send(HeartbeatCommand::Acked).await;
                Ok(false)
            }
            RawMessage::Close => {
                tracing::info!("closing connection");
                Ok(true)
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
