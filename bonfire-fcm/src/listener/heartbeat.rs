use std::io;
use std::pin::Pin;
use std::time::Duration;

use tokio::sync::mpsc;
use tokio::time::{Instant, Sleep, sleep};
use tokio_util::sync::CancellationToken;
use tracing::instrument;

use crate::models::RawMessage;
use crate::{Connection, Error, Result, proto};

// Connection idle time before sending HeartbeatPing
const HEARTBEAT_DURATION: Duration = Duration::from_secs(60);
// How long to wait for a HeartbeatAck before closing the connection
const ACK_DURATION: Duration = Duration::from_secs(15);

pub(super) enum Command {
    // Server sent HeartbeatPing
    Pinged { status: Option<i64> },
    // Server sent HeartbeatAck
    Acked,
    // Server sent other type of message
    MessageReceived,
}

// TCP keepalive probes may be handled by proxies and never reach the server, heartbeats solve this
// problem as they can only be processed by the server
pub(super) struct Heartbeat;
impl Heartbeat {
    #[instrument(
        name = "heartbeat",
        skip(command_receiver, connection, cancellation_token)
    )]
    pub(super) async fn task(
        mut command_receiver: mpsc::Receiver<Command>,
        connection: Connection,
        cancellation_token: CancellationToken,
        id: u64,
    ) -> Result<()> {
        let mut ack_timer = Option::<Pin<Box<Sleep>>>::None;
        let heartbeat_timer = sleep(HEARTBEAT_DURATION);
        tokio::pin!(heartbeat_timer);

        loop {
            tokio::select! {
                () = cancellation_token.cancelled() => {
                    tracing::debug!("stopping");
                    break;
                }
                command = command_receiver.recv() => {
                    match command {
                        Some(Command::Pinged { status }) => {
                            Self::ack(&connection, status).await?;
                        }
                        Some(Command::Acked) => {
                            ack_timer = None;
                        }
                        Some(Command::MessageReceived) => {}
                        None => break,
                    }

                    heartbeat_timer
                        .as_mut()
                        .reset(Instant::now() + HEARTBEAT_DURATION);
                }
                () = &mut heartbeat_timer, if ack_timer.is_none() => {
                    Self::ping(&connection).await?;
                    ack_timer = Some(Box::pin(sleep(ACK_DURATION)));
                }
                () = async {
                    if let Some(ref mut timer) = ack_timer {
                        timer.await;
                    } else {
                        // Future that never resolves
                        std::future::pending::<()>().await;
                    }
                } => {
                    tracing::error!("timed out waiting for an ack");
                    return Err(Error::IoError(io::ErrorKind::TimedOut.into()));
                }
            }
        }
        Ok(())
    }

    async fn ping(connection: &Connection) -> Result<()> {
        let ping = proto::HeartbeatPing::default();

        tracing::debug!("pinging");
        connection
            .write(RawMessage::HeartbeatPing(ping))
            .await
            .inspect_err(|error| tracing::error!(?error, "failed to ping"))?;
        Ok(())
    }

    async fn ack(connection: &Connection, status: Option<i64>) -> Result<()> {
        let ack = proto::HeartbeatAck {
            status,
            ..Default::default()
        };

        tracing::debug!("acking heartbeat");
        connection
            .write(RawMessage::HeartbeatAck(ack))
            .await
            .inspect_err(|error| tracing::error!(?error, "failed to ack heartbeat"))?;
        Ok(())
    }
}
