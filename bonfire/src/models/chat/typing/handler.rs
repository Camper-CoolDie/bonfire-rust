use tokio::sync::mpsc;
use tokio::time::{Instant, Interval, MissedTickBehavior, interval_at};
use tracing::Instrument as _;

use crate::Client;
use crate::client::Request as _;
use crate::models::chat::Tag;
use crate::models::chat::typing::Command;
use crate::requests::chat::NotifyTypingRequest;

/// Manages a background task that sends periodic typing notifications to a chat.
///
/// This struct acts as a handle to the background task. When the `TypingHandler` is dropped, the
/// typing task is automatically signaled to stop.
#[derive(Clone)]
pub struct Handler {
    command_sender: mpsc::Sender<Command>,
}
impl Handler {
    pub(super) fn spawn(client: &Client, tag: Tag) -> Self {
        let (command_sender, command_receiver) = mpsc::channel(1);
        let client = client.clone();
        let span = tracing::info_span!("typing", ?tag);

        let task = async move {
            let request = NotifyTypingRequest::new(tag);
            let mut interval = interval_at(Instant::now(), NotifyTypingRequest::PERIOD);
            interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

            Self::task_loop(&client, &request, interval, command_receiver).await;
        };
        tokio::spawn(task.instrument(span));

        Self { command_sender }
    }

    async fn task_loop(
        client: &Client,
        request: &NotifyTypingRequest,
        mut interval: Interval,
        mut command_receiver: mpsc::Receiver<Command>,
    ) {
        let mut is_paused = false;

        loop {
            tokio::select! {
                command = command_receiver.recv() => {
                    match command {
                        Some(Command::Pause) => {
                            tracing::debug!("pausing");
                            is_paused = true;
                        }
                        Some(Command::Resume) => {
                            tracing::debug!("resuming");
                            is_paused = false;
                        }
                        Some(Command::Cancel) | None => {
                            tracing::debug!("stopping");
                            break;
                        }
                    }
                }
                _ = interval.tick(), if !is_paused => {
                    let _ = request.send_request(client).await;
                }
            }
        }
    }

    /// Sends a command to the background task to pause sending typing notifications.
    pub async fn pause(&self) {
        let _ = self.command_sender.send(Command::Pause).await;
    }

    /// Sends a command to the background task to resume sending typing notifications.
    pub async fn resume(&self) {
        let _ = self.command_sender.send(Command::Resume).await;
    }

    /// Sends a command to the background task to stop sending typing notifications, consuming this
    /// `TypingHandler`.
    pub async fn cancel(self) {
        let _ = self.command_sender.send(Command::Cancel).await;
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        // If .try_send() returns an error (the buffer is full), the task will eventually fail and
        // stop because of disconnection. Though for an immediate effect we try to send the Cancel
        // command
        let _ = self.command_sender.try_send(Command::Cancel);
    }
}
