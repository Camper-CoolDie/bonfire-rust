#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Credentials {
    pub android_id: u64,
    pub security_token: u64,
    pub gcm_token: String,
    pub installation_id: String,
    pub installation_auth_token: String,
    pub installation_refresh_token: String,
}
