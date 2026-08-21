use crate::Error;
use crate::models::Subscription;

#[derive(Debug)]
pub enum Message {
    Data(serde_json::Value),
    // Some previous notifications were lost before they could reach us, the caller needs to
    // manually fetch all missed notifications from the backend
    MessagesDeleted {
        count: usize,
    },
    ListenerStopped {
        subscription: Subscription,
        error: Option<Error>,
    },
}
