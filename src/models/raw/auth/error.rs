use serde::Deserialize;

use crate::models::auth::{TfaKind, TfaRequired};

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum RawTfaKind {
    Totp,
    EmailLink,
}

impl From<RawTfaKind> for TfaKind {
    fn from(value: RawTfaKind) -> Self {
        match value {
            RawTfaKind::Totp => TfaKind::Totp,
            RawTfaKind::EmailLink => TfaKind::EmailLink,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct RawTfaRequired {
    #[serde(rename = "tfaType")]
    kind: RawTfaKind,
    #[serde(rename = "tfaWaitToken")]
    wait_token: String,
}

impl From<RawTfaRequired> for TfaRequired {
    fn from(value: RawTfaRequired) -> Self {
        Self {
            kind: value.kind.into(),
            wait_token: value.wait_token,
        }
    }
}
