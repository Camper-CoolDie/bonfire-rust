#[derive(Debug)]
pub(crate) enum Kind {
    HeartbeatPing,
    HeartbeatAck,
    LoginRequest,
    LoginResponse,
    Close,
    IqStanza,
    DataMessageStanza,
    Unknown(u8),
}

impl From<Kind> for u8 {
    fn from(value: Kind) -> Self {
        match value {
            Kind::HeartbeatPing => 0,
            Kind::HeartbeatAck => 1,
            Kind::LoginRequest => 2,
            Kind::LoginResponse => 3,
            Kind::Close => 4,
            Kind::IqStanza => 7,
            Kind::DataMessageStanza => 8,
            Kind::Unknown(unknown) => unknown,
        }
    }
}

impl From<u8> for Kind {
    fn from(value: u8) -> Self {
        match value {
            0 => Kind::HeartbeatPing,
            1 => Kind::HeartbeatAck,
            2 => Kind::LoginRequest,
            3 => Kind::LoginResponse,
            4 => Kind::Close,
            7 => Kind::IqStanza,
            8 => Kind::DataMessageStanza,
            other => Kind::Unknown(other),
        }
    }
}
