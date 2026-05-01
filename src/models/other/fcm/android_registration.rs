#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Represents Android registration data needed for FCM token generation.
///
/// This struct contains the authentication tokens and identifiers obtained from Google Cloud
/// Messaging services.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(docsrs, doc(cfg(feature = "fcm")))]
pub struct AndroidRegistration {
    /// The Firebase installations auth token
    pub installation_auth_token: String,
    /// The Android device ID
    pub android_id: u64,
    /// The security token for Android device authentication
    pub security_token: u64,
    /// The GCM (Google Cloud Messaging) token
    pub gcm_token: String,
}
