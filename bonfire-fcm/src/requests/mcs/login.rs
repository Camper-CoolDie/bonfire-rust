use std::collections::VecDeque;

use crate::models::RawMessage;
use crate::{Connection, Error, Result, proto};

pub(crate) struct LoginRequest<'a> {
    android_id: u64,
    security_token: u64,
    persistent_ids: &'a VecDeque<String>,
}
impl<'a> LoginRequest<'a> {
    pub(crate) fn new(
        android_id: u64,
        security_token: u64,
        persistent_ids: &'a VecDeque<String>,
    ) -> Self {
        Self {
            android_id,
            security_token,
            persistent_ids,
        }
    }

    pub(crate) async fn send_message(&self, connection: &Connection) -> Result<()> {
        let android_id = self.android_id.to_string();
        let security_token = self.security_token.to_string();

        let request = proto::LoginRequest {
            id: "chrome-63.0.3234.0".to_owned(),
            domain: "mcs.android.com".to_owned(),
            user: android_id.clone(),
            resource: android_id,
            auth_token: security_token,
            device_id: Some(format!("android-{:x}", self.android_id)),
            setting: vec![proto::Setting {
                name: "new_vc".to_owned(),
                value: "1".to_owned(),
            }],
            received_persistent_id: self.persistent_ids.iter().cloned().collect(),
            adaptive_heartbeat: Some(false),
            use_rmq2: Some(true),
            auth_service: Some(2),
            network_type: Some(1),
            ..Default::default()
        };

        connection.write_version().await?;
        connection.write(RawMessage::LoginRequest(request)).await?;

        connection.read_and_check_version().await?;
        let response = match connection.read().await? {
            Some(RawMessage::LoginResponse(response)) => response,
            Some(other) => {
                return Err(Error::McsProtocolError(format!(
                    "unexpected message: {:?}",
                    other.kind()
                )));
            }
            None => {
                return Err(Error::McsProtocolError(
                    "expected login response".to_owned(),
                ));
            }
        };

        match response.error {
            Some(error) => Err(Error::McsLoginError {
                code: error.code,
                message: error.message,
                kind: error.r#type,
            }),
            None => Ok(()),
        }
    }
}
