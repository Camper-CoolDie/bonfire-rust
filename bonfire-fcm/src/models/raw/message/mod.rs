mod kind;

use bytes::Bytes;
pub(crate) use kind::Kind;

use crate::proto;

pub(crate) enum Message {
    Data { persistent_id: String, body: Bytes },
    MessagesDeleted { persistent_id: String, count: usize },
    LoginRequest(proto::LoginRequest),
    LoginResponse(proto::LoginResponse),
    HeartbeatPing(proto::HeartbeatPing),
    HeartbeatAck(proto::HeartbeatAck),
    // The server is manually closing the connection (shouldn't reconnect)
    Close,
}
impl Message {
    pub(crate) fn kind(&self) -> Kind {
        match self {
            Message::Data { .. } | Message::MessagesDeleted { .. } => Kind::DataMessageStanza,
            Message::LoginRequest(_) => Kind::LoginRequest,
            Message::LoginResponse(_) => Kind::LoginResponse,
            Message::HeartbeatPing(_) => Kind::HeartbeatPing,
            Message::HeartbeatAck(_) => Kind::HeartbeatAck,
            Message::Close => Kind::Close,
        }
    }
}
