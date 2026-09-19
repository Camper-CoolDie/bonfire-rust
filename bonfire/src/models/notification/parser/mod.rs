mod data;

use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::Utc;
use data::Data;
use fcm::Parse;
use fcm::models::{Message, Subscription};
use tokio::sync::oneshot;

use crate::models::Notification;
use crate::requests::raw::RawNotification;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Parser {
    subscription_sender: oneshot::Sender<(Subscription, Option<Error>)>,
    started_at: i64,
    dropped_count: AtomicUsize,
}
impl Parser {
    #[must_use]
    pub fn new() -> (Self, oneshot::Receiver<(Subscription, Option<Error>)>) {
        let (subscription_sender, subscription_receiver) = oneshot::channel();

        (
            Self {
                subscription_sender,
                started_at: Utc::now().timestamp_millis(),
                dropped_count: AtomicUsize::new(0),
            },
            subscription_receiver,
        )
    }
}

impl Parse for Parser {
    type Target = Notification;
    type Error = Error;

    async fn parse(&self, message: Message) -> Result<Option<Notification>> {
        let data = match message {
            Message::Data(data) => serde_json::from_value::<Data>(data)?,
            Message::MessagesDeleted { count } => {
                self.dropped_count.fetch_add(count, Ordering::AcqRel);
                return Ok(None);
            }
        };

        let Data::Legacy(payload) = data;
        let notification = serde_json::from_str::<RawNotification>(&payload)?;

        // The server may cache notifications when disconnected and send them after connecting. They
        // must be dropped
        if notification.sent_at < self.started_at {
            self.dropped_count.fetch_add(1, Ordering::AcqRel);
            return Ok(None);
        }

        let dropped_count = self.dropped_count.swap(0, Ordering::AcqRel);
        if dropped_count > 0 {
            tracing::warn!("dropped {dropped_count} notifications");
        }

        tracing::info!(kind = ?notification.kind, "parsed notification");
        Ok(Some(notification.try_into()?))
    }

    async fn stop(self, subscription: Subscription, error: Option<Error>) {
        // Allow cases when caller intentionally drops the receiver
        let _ = self.subscription_sender.send((subscription, error));
    }
}
