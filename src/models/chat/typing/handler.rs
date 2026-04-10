use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::{Instant, Interval, MissedTickBehavior, interval_at};
use tracing::Instrument;

use crate::Client;
use crate::client::Request as _;
use crate::models::chat::Tag;
use crate::models::chat::typing::Command;
use crate::requests::chat::NotifyTypingRequest;

/// Manages a background task that sends periodic typing notifications to a chat.
///
/// This struct acts as a handle to the background task. When the `TypingHandler` is dropped, the
/// typing task is automatically signaled to stop.
pub struct Handler {
    sender: mpsc::Sender<Command>,
    _task: JoinHandle<()>,
}
impl Handler {
    pub(super) fn spawn(client: &Client, tag: Tag) -> Self {
        let (sender, receiver) = mpsc::channel(1);
        let client = client.clone();
        let span = tracing::info_span!("typing_task", ?tag);

        let task = async move {
            let request = NotifyTypingRequest::new(tag);
            let mut interval = interval_at(Instant::now(), NotifyTypingRequest::PERIOD);
            interval.set_missed_tick_behavior(MissedTickBehavior::Delay);

            Self::task_loop(&client, &request, interval, receiver).await;
        }
        .instrument(span);

        Self {
            sender,
            _task: tokio::spawn(task),
        }
    }

    async fn task_loop(
        client: &Client,
        request: &NotifyTypingRequest,
        mut interval: Interval,
        mut receiver: mpsc::Receiver<Command>,
    ) {
        let mut is_paused = false;

        loop {
            tokio::select! {
                _ = interval.tick(), if !is_paused => {
                    let _ = request.send_request(client).await;
                }
                command = receiver.recv() => {
                    match command {
                        Some(Command::Pause) => {
                            tracing::debug!("paused");
                            is_paused = true;
                        }
                        Some(Command::Resume) => {
                            tracing::debug!("resumed");
                            is_paused = false;
                        }
                        Some(Command::Stop) | None => {
                            tracing::debug!("stopped");
                            break;
                        }
                    }
                }
            }
        }
    }

    /// Sends a command to the background task to pause sending typing notifications.
    pub async fn pause(&self) {
        let _ = self.sender.send(Command::Pause).await;
    }

    /// Sends a command to the background task to resume sending typing notifications.
    pub async fn resume(&self) {
        let _ = self.sender.send(Command::Resume).await;
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        // If .try_send() returns an error (the buffer is full), the task will eventually fail and
        // stop because of disconnection. Though for an immediate effect we try to send the Stop
        // command
        let _ = self.sender.try_send(Command::Stop);
    }
}
