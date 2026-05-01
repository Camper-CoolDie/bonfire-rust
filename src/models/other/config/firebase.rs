const DEFAULT_PROJECT_ID: &str = "bonfire-sit";
const DEFAULT_APP_ID: &str = "1:778141366343:web:8d59a00829b37c6949d962";
const DEFAULT_API_KEY: &str = "AIzaSyDDw53XejyeVlnYEcz8s6DL8cD2soLlY2g";
const DEFAULT_VAPID_KEY: &str =
    "BK897KkbZKz2V_euNvaMkNLCePpDwmSRpZqPYNaS5PkCuC2zcw_EbZGSynJ2BkPWiS1EufTRpuWEethjw31pjvA";

/// Represents the Firebase configuration for FCM push notifications.
///
/// This struct holds the necessary credentials to communicate with Google's Firebase Cloud
/// Messaging service for push notification support.
#[derive(Clone, Debug)]
pub struct Firebase {
    /// The Firebase project ID
    pub project_id: String,
    /// The Firebase application ID
    pub app_id: String,
    /// The Firebase API key
    pub api_key: String,
    /// The VAPID (Voluntary Application Server Identification) public key for web push
    /// notifications
    pub vapid_key: Option<String>,
}

impl Default for Firebase {
    fn default() -> Self {
        Self {
            project_id: DEFAULT_PROJECT_ID.to_string(),
            app_id: DEFAULT_APP_ID.to_string(),
            api_key: DEFAULT_API_KEY.to_string(),
            vapid_key: Some(DEFAULT_VAPID_KEY.to_string()),
        }
    }
}
