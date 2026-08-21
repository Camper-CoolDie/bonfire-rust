const PROJECT_ID: &str = "bonfire-sit";
const APP_ID: &str = "1:778141366343:web:8d59a00829b37c6949d962";
const API_KEY: &str = "AIzaSyDDw53XejyeVlnYEcz8s6DL8cD2soLlY2g";
const VAPID_KEY: &str =
    "BK897KkbZKz2V_euNvaMkNLCePpDwmSRpZqPYNaS5PkCuC2zcw_EbZGSynJ2BkPWiS1EufTRpuWEethjw31pjvA";
const URI: &str = "https://bonfire.moe";

#[derive(Debug)]
pub(crate) struct Config {
    pub project_id: String,
    pub app_id: String,
    pub api_key: String,
    pub vapid_key: String,
    pub uri: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            project_id: PROJECT_ID.to_string(),
            app_id: APP_ID.to_string(),
            api_key: API_KEY.to_string(),
            vapid_key: VAPID_KEY.to_string(),
            uri: URI.to_string(),
        }
    }
}
