#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::AndroidRegistration;

/// Represents FCM credentials needed for receiving push notifications.
///
/// This struct contains the FCM token and encryption keys required to decrypt incoming push
/// notifications from Firebase Cloud Messaging.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(docsrs, doc(cfg(feature = "fcm")))]
pub struct Credentials {
    /// The FCM registration token used to receive push notifications
    pub token: String,
    /// The base64-encoded P-256 public key for ECE encryption
    pub public_key: String,
    /// The base64-encoded P-256 private key for ECE encryption
    pub private_key: String,
    /// The base64-encoded authentication secret for ECE encryption
    pub auth_secret: String,
    /// The Android registration data
    pub android: AndroidRegistration,
}
