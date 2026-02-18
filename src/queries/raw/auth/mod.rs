mod error;

pub(crate) use error::*;
use serde::Deserialize;

use crate::models::Auth;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawAuth {
    access_token: String,
    refresh_token: String,
}

impl From<RawAuth> for Auth {
    fn from(value: RawAuth) -> Self {
        Self {
            access_token: value.access_token,
            refresh_token: value.refresh_token,
        }
    }
}
