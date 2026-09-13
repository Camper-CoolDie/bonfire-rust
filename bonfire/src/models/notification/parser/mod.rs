mod data;

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use chrono::Utc;
use data::Data;
use fcm::Parse;
use fcm::models::{Message, Subscription};

use crate::models::Notification;
use crate::requests::raw::RawNotification;
use crate::{Error, Result};

#[derive(Debug)]
struct Inner<C, Fut>
where
    C: Fn(Subscription, Option<Error>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    callback: C,
    started_at: i64,
    dropped_count: AtomicUsize,
}

#[derive(Clone, Debug)]
pub struct Parser<C, Fut>
where
    C: Fn(Subscription, Option<Error>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    inner: Arc<Inner<C, Fut>>,
}
impl<C, Fut> Parser<C, Fut>
where
    C: Fn(Subscription, Option<Error>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    pub fn new(callback: C) -> Self {
        Self {
            inner: Arc::new(Inner {
                callback,
                started_at: Utc::now().timestamp_millis(),
                dropped_count: AtomicUsize::new(0),
            }),
        }
    }
}

impl<C, Fut> Parse for Parser<C, Fut>
where
    C: Fn(Subscription, Option<Error>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    type Target = Notification;
    type Error = Error;

    async fn parse(&self, message: Message) -> Result<Option<Notification>> {
        let data = match message {
            Message::Data(data) => serde_json::from_value::<Data>(data)?,
            Message::MessagesDeleted { count } => {
                self.inner.dropped_count.fetch_add(count, Ordering::AcqRel);
                return Ok(None);
            }
        };

        let Data::Legacy(payload) = data;
        let notification = serde_json::from_str::<RawNotification>(&payload)?;

        // The server may cache notifications when disconnected and send them after connecting. They
        // must be dropped
        if notification.sent_at < self.inner.started_at {
            self.inner.dropped_count.fetch_add(1, Ordering::AcqRel);
            return Ok(None);
        }

        let dropped_count = self.inner.dropped_count.swap(0, Ordering::AcqRel);
        if dropped_count > 0 {
            tracing::info!("dropped {dropped_count} notifications");
        }

        tracing::debug!(kind = ?notification.kind, "received notification");
        Ok(Some(notification.try_into()?))
    }

    async fn stop(&self, subscription: Subscription, error: Option<Error>) {
        (self.inner.callback)(subscription, error).await;
    }
}
