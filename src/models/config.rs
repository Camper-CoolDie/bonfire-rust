const DEFAULT_ANYTHING_FANDOM_ID: u64 = 1627;
const DEFAULT_ANYTHING_FANDOM_NAME: &str = "Anything";

/// Represents the default configuration values for the Bonfire API client.
#[derive(Clone, Debug)]
pub struct Config {
    /// A list of account IDs that have protoadmin privileges.
    ///
    /// This list is fetched from the server via
    /// [`Client::get_initial_data`][crate::Client::get_initial_data] and is empty by default.
    pub protoadmin_ids: Vec<u64>,
    /// The default fandom ID to use for quick post creation
    pub quick_post_fandom_id: u64,
    /// The default fandom name to use for quick post creation
    pub quick_post_fandom_name: String,
    /// The default fandom ID to use for quick image publication
    pub quick_image_fandom_id: u64,
    /// The default fandom name to use for quick image publication
    pub quick_image_fandom_name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            protoadmin_ids: Vec::new(),
            quick_post_fandom_id: DEFAULT_ANYTHING_FANDOM_ID,
            quick_post_fandom_name: DEFAULT_ANYTHING_FANDOM_NAME.to_string(),
            quick_image_fandom_id: DEFAULT_ANYTHING_FANDOM_ID,
            quick_image_fandom_name: DEFAULT_ANYTHING_FANDOM_NAME.to_string(),
        }
    }
}
