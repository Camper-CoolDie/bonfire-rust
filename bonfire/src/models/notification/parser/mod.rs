mod data;

use b_fcm::Parse;
use b_fcm::models::{Message, Subscription};
use chrono::Utc;
use data::Data;
use tokio::sync::oneshot;

use crate::models::Notification;
use crate::requests::raw::RawNotification;
use crate::{Error, Result};

#[derive(Debug)]
pub struct Parser {
    subscription_sender: oneshot::Sender<(Subscription, Option<b_fcm::Error>)>,
    started_at: i64,
    dropped_count: usize,
}
impl Parser {
    #[must_use]
    pub fn new() -> (
        Self,
        oneshot::Receiver<(Subscription, Option<b_fcm::Error>)>,
    ) {
        let (subscription_sender, subscription_receiver) = oneshot::channel();

        (
            Self {
                subscription_sender,
                started_at: Utc::now().timestamp_millis(),
                dropped_count: 0,
            },
            subscription_receiver,
        )
    }
}

impl Parse for Parser {
    type Target = Notification;
    type Error = Error;

    fn parse(&mut self, message: Message) -> Result<Option<Notification>> {
        let data = match message {
            Message::Data(data) => serde_json::from_value::<Data>(data)?,
            Message::MessagesDeleted { count } => {
                self.dropped_count += count;
                return Ok(None);
            }
        };

        let Data::Legacy(payload) = data;
        let notification = serde_json::from_str::<RawNotification>(&payload)?;

        // The server may cache notifications when disconnected and send them after connecting.
        // Those notifications must be dropped
        if notification.sent_at < self.started_at {
            self.dropped_count += 1;
            return Ok(None);
        }

        if self.dropped_count > 0 {
            tracing::warn!("dropped {} notifications", self.dropped_count);
            self.dropped_count = 0;
        }

        tracing::info!(kind = ?notification.kind, "parsed notification");
        Ok(Some(notification.try_into()?))
    }

    fn stop(self, subscription: Subscription, error: Option<b_fcm::Error>) {
        if self.dropped_count > 0 {
            tracing::warn!("dropped {} notifications", self.dropped_count);
        }

        // Allow cases when caller intentionally drops the receiver
        let _ = self.subscription_sender.send((subscription, error));
    }
}
